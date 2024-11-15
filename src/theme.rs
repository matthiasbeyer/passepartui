use ratatui::style::{palette::tailwind, Color};

#[derive(Debug, Default, Clone, Copy)]
pub struct Theme {
    pub button_keyboard_label: Color,
    pub button_label: Color,
    pub debug: Color,
    pub details_border: Color,
    pub details_field_fg: Color,
    pub details_hint_fg: Color,
    pub menu_bg: Color,
    pub menu_button_background: Color,
    pub menu_button_highlight: Color,
    pub menu_button_keyboard_label: Color,
    pub menu_button_label: Color,
    pub menu_button_shadow: Color,
    pub menu_logo_fg: Color,
    pub popup_border: Color,
    pub search_bg: Color,
    pub search_border: Color,
    pub standard_bg: Color,
    pub standard_fg: Color,
    pub status_bar_bg: Color,
    pub status_bar_fg: Color,
    pub table_alt_row: Color,
    pub table_buffer_bg: Color,
    pub table_header_bg: Color,
    pub table_header_fg: Color,
    pub table_normal_row: Color,
    pub table_pattern_highlight_bg: Color,
    pub table_row_fg: Color,
    pub table_selected_cell_style_fg: Color,
    pub table_selected_column_style_fg: Color,
    pub table_selected_row_style_fg: Color,
}

impl Theme {
    pub fn new() -> Self {
        let palette = &tailwind::CYAN;
        Self {
            button_keyboard_label: tailwind::SLATE.c400,
            button_label: tailwind::SLATE.c300,
            debug: tailwind::BLUE.c500,
            details_border: palette.c950,
            details_field_fg: tailwind::SLATE.c200,
            details_hint_fg: tailwind::SLATE.c400,
            menu_bg: palette.c950,
            menu_button_background: palette.c900,
            menu_button_highlight: palette.c800,
            menu_button_keyboard_label: tailwind::SLATE.c400,
            menu_button_label: tailwind::SLATE.c300,
            menu_button_shadow: palette.c950,
            menu_logo_fg: palette.c600,
            popup_border: palette.c700,
            search_bg: tailwind::SLATE.c900,
            search_border: palette.c400,
            standard_bg: tailwind::SLATE.c900,
            standard_fg: tailwind::SLATE.c200,
            status_bar_bg: palette.c950,
            status_bar_fg: tailwind::SLATE.c200,
            table_alt_row: tailwind::SLATE.c900,
            table_buffer_bg: tailwind::SLATE.c900,
            table_header_bg: tailwind::BLUE.c900,
            table_header_fg: tailwind::SLATE.c200,
            table_normal_row: tailwind::SLATE.c950,
            table_pattern_highlight_bg: palette.c700,
            table_row_fg: tailwind::SLATE.c200,
            table_selected_cell_style_fg: tailwind::BLUE.c600,
            table_selected_column_style_fg: tailwind::BLUE.c400,
            table_selected_row_style_fg: tailwind::BLUE.c400,
        }
    }
}
