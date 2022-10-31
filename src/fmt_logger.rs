// logger of wangxiaoyu
use ansi_term::{Colour, Style};
use std::fmt;
use tracing_core::{Event, Field, Level, Subscriber};
use tracing_subscriber::field::{VisitFmt, VisitOutput};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::{time, FormatTime};
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{field, EnvFilter};

pub fn init_loggers() {
    let _subscriber = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .event_format(MessageFormat::new(true))
        .init();
}

// ==========================================================
//
// Format
//
// ===========================================================
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct MessageFormat {
    is_ansi: bool,
}

impl MessageFormat {
    fn new(is_ansi: bool) -> Self {
        Self { is_ansi }
    }

    // format of message in event
    fn style_for(level: &Level) -> Style {
        match *level {
            Level::TRACE => Style::new().fg(Colour::Purple),
            Level::DEBUG => Style::new().fg(Colour::Blue),
            Level::INFO => Style::new().fg(Colour::Green),
            Level::WARN => Style::new().fg(Colour::Yellow),
            Level::ERROR => Style::new().fg(Colour::Red),
        }
    }

    fn format_timestamp(&self, writer: &mut Writer<'_>) -> fmt::Result {
        // Using tracing-subscriber::fmt::time module deal time stamp
        let timer = time();

        if self.is_ansi {
            let style = Style::new().dimmed();
            write!(writer, "{}", style.prefix())?;

            if timer.format_time(writer).is_err() {
                writer.write_str("<unknown time>")?;
            }

            write!(writer, "{} ", style.suffix())?;
            return Ok(());
        }

        if timer.format_time(writer).is_err() {
            writer.write_str("<unknown time>")?;
        }

        writer.write_char(' ')
    }
}

// ==========================================================
//
// LEVEL LABEL itself
//
// ===========================================================
const TRACE_STR: &str = "TRACE";
const DEBUG_STR: &str = "DEBUG";
const INFO_STR: &str = "INFO";
const WARN_STR: &str = "WARN";
const ERROR_STR: &str = "ERROR";

struct FmtLevel<'a> {
    level: &'a Level,
    is_ansi: bool,
}

impl<'a> FmtLevel<'a> {
    fn new(level: &'a Level, is_ansi: bool) -> Self {
        Self { level, is_ansi }
    }
}

impl<'a> fmt::Display for FmtLevel<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_ansi {
            match *self.level {
                Level::TRACE => write!(f, "{}", Colour::Purple.underline().paint(TRACE_STR)),
                Level::DEBUG => write!(f, "{}", Colour::Blue.underline().paint(DEBUG_STR)),
                Level::INFO => write!(f, "{} ", Colour::Green.underline().paint(INFO_STR)),
                Level::WARN => write!(f, "{} ", Colour::Yellow.underline().paint(WARN_STR)),
                Level::ERROR => write!(f, "{}", Colour::Red.underline().paint(ERROR_STR)),
            }
        } else {
            match *self.level {
                Level::TRACE => f.pad(TRACE_STR),
                Level::DEBUG => f.pad(DEBUG_STR),
                Level::INFO => f.pad(INFO_STR),
                Level::WARN => f.pad(WARN_STR),
                Level::ERROR => f.pad(ERROR_STR),
            }
        }
    }
}

// ==========================================================
//
// Define Visitor
//
// ===========================================================

struct MyVisitor<'a> {
    writer: Writer<'a>,
    is_empty: bool,
    style: Style,
    result: fmt::Result,
}

impl<'a> MyVisitor<'a> {
    pub fn new(writer: Writer<'a>, is_empty: bool) -> Self {
        Self {
            writer,
            is_empty,
            style: Style::default(),
            result: Ok(()),
        }
    }

    fn with_style(self, style: Style) -> Self {
        Self { style, ..self }
    }

    fn write_padded(&mut self, value: &impl fmt::Debug) {
        let padding = if self.is_empty {
            self.is_empty = false;
            ""
        } else {
            ", \n    "
        };
        self.result = write!(self.writer, "{}{:?}", padding, value);
    }

    fn bold(&self) -> Style {
        if self.writer.has_ansi_escapes() {
            self.style.bold()
        } else {
            Style::new()
        }
    }
}

impl<'a> field::Visit for MyVisitor<'a> {
    fn record_str(&mut self, field: &Field, value: &str) {
        if self.result.is_err() {
            return;
        }

        if field.name() == "message" {
            self.record_debug(field, &format_args!("{}", value))
        } else {
            self.record_debug(field, &value)
        }
    }

    fn record_error(&mut self, field: &Field, value: &(dyn std::error::Error + 'static)) {
        if let Some(_source) = value.source() {
            let bold = self.bold();
            self.record_debug(
                field,
                &format_args!(
                    "{}, {}{}.sources{}: {}",
                    value,
                    bold.prefix(),
                    field,
                    bold.infix(self.style),
                    // ErrorSourceList(source),
                    "Here".to_string()
                ),
            )
        } else {
            self.record_debug(field, &format_args!("{}", value))
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if self.result.is_err() {
            return;
        }
        let bold = self.bold();
        match field.name() {
            "message" => self.write_padded(&format_args!("{}{:?}", self.style.prefix(), value,)),
            // Skip fields that are actually log metadata that have already been handled
            #[cfg(feature = "tracing-log")]
            name if name.starts_with("log.") => self.result = Ok(()),
            name if name.starts_with("r#") => self.write_padded(&format_args!(
                "{}{}{}: {:?}",
                bold.prefix(),
                &name[2..],
                bold.infix(self.style),
                value
            )),
            name => self.write_padded(&format_args!(
                "{}{}{}: {:?}",
                bold.prefix(),
                name,
                bold.infix(self.style),
                value
            )),
        };
    }
}

impl<'a> VisitOutput<fmt::Result> for MyVisitor<'a> {
    fn finish(mut self) -> fmt::Result {
        write!(&mut self.writer, "{}", self.style.suffix())?;
        self.result
    }
}

impl<'a> VisitFmt for MyVisitor<'a> {
    fn writer(&mut self) -> &mut dyn fmt::Write {
        &mut self.writer
    }
}

// ==========================================================
//
// FormatEvent
//
// ===========================================================
impl<C, N> FormatEvent<C, N> for MessageFormat
    where
        C: Subscriber + for<'a> LookupSpan<'a>,
        N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &FmtContext<'_, C, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        // init
        let (dimmed, file_style, thread_style) = if self.is_ansi {
            (
                Style::new().dimmed().italic(),
                Style::new().underline().fg(Colour::Cyan),
                Style::new().on(Colour::RGB(100, 100, 100)),
            )
        } else {
            (Style::new(), Style::new(), Style::new())
        };

        let meta = event.metadata();

        // time
        write!(&mut writer, "  ")?;
        self.format_timestamp(&mut writer)?;

        // Level
        let style = MessageFormat::style_for(meta.level());
        write!(writer, "{} ", FmtLevel::new(meta.level(), self.is_ansi))?;

        // target
        let target_style = if self.is_ansi { style.bold() } else { style };
        write!(
            writer,
            "{}{}{}:",
            target_style.prefix(),
            meta.target(),
            target_style.infix(style)
        )?;

        // line number
        let line_number = match meta.line() {
            Some(line_number) => {
                write!(
                    writer,
                    "{}{}{} ",
                    style.prefix(),
                    line_number,
                    style.infix(style)
                )?;
                line_number.to_string()
            }
            None => "".to_string(),
        };

        // ==========================================================
        //
        // Visitor
        //
        // ===========================================================
        writer.write_char('\n')?;

        // file
        if let Some(file) = meta.file() {
            write!(
                writer,
                "    {} {}",
                dimmed.paint("at"),
                file_style.paint(file),
            )?;
            write!(writer, ":{}", file_style.paint(line_number))?;

            if file.len() < 50 {
                writer.write_char(' ')?;
            } else {
                writer.write_str("\n    ")?;
            }
        } else{
            writer.write_str("    ")?;
        }

        // thread
        write!(writer, "{} ", dimmed.paint("on"))?;
        let thread = std::thread::current();
        let thread_name = match thread.name() {
            None => "",
            Some(name) => name,
        };

        let thread_string = thread_style.paint(format!("{} {:?}", thread_name, thread.id()));
        write!(writer, "{}", thread_string)?;
        writer.write_char('\n')?;

        // message
        writer.write_str("    ")?;
        let mut v = MyVisitor::new(writer.by_ref(), true).with_style(style);
        event.record(&mut v);
        v.finish()?;

        writer.write_str("\n\n")
    }
}
