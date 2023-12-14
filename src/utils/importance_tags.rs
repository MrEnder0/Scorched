use owo_colors::{
    colors::{css::*, CustomColor},
    ComboColorDisplay, OwoColorize,
};

pub(crate) fn error_tag(
) -> ComboColorDisplay<'static, CustomColor<0, 0, 0>, CustomColor<{ u8::MAX }, 0, 0>, &'static str>
{
    "[ERROR]".fg::<Black>().bg::<Red>()
}

pub(crate) fn warning_tag() -> ComboColorDisplay<
    'static,
    CustomColor<0, 0, 0>,
    CustomColor<{ u8::MAX }, { u8::MAX }, 0>,
    &'static str,
> {
    "[WARNING]".fg::<Black>().bg::<Yellow>()
}

pub(crate) fn info_tag(
) -> ComboColorDisplay<'static, CustomColor<0, 0, 0>, CustomColor<211, 211, 211>, &'static str> {
    "[INFO]".fg::<Black>().bg::<LightGray>()
}

pub(crate) fn debug_tag() -> ComboColorDisplay<
    'static,
    CustomColor<0, 0, 0>,
    CustomColor<{ u8::MAX }, 0, { u8::MAX }>,
    &'static str,
> {
    "[DEBUG]".fg::<Black>().bg::<Magenta>()
}
