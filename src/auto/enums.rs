// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "AtspiRole")]
pub enum Role {
    #[doc(alias = "ATSPI_ROLE_INVALID")]
    Invalid,
    #[doc(alias = "ATSPI_ROLE_ACCELERATOR_LABEL")]
    AcceleratorLabel,
    #[doc(alias = "ATSPI_ROLE_ALERT")]
    Alert,
    #[doc(alias = "ATSPI_ROLE_ANIMATION")]
    Animation,
    #[doc(alias = "ATSPI_ROLE_ARROW")]
    Arrow,
    #[doc(alias = "ATSPI_ROLE_CALENDAR")]
    Calendar,
    #[doc(alias = "ATSPI_ROLE_CANVAS")]
    Canvas,
    #[doc(alias = "ATSPI_ROLE_CHECK_BOX")]
    CheckBox,
    #[doc(alias = "ATSPI_ROLE_CHECK_MENU_ITEM")]
    CheckMenuItem,
    #[doc(alias = "ATSPI_ROLE_COLOR_CHOOSER")]
    ColorChooser,
    #[doc(alias = "ATSPI_ROLE_COLUMN_HEADER")]
    ColumnHeader,
    #[doc(alias = "ATSPI_ROLE_COMBO_BOX")]
    ComboBox,
    #[doc(alias = "ATSPI_ROLE_DATE_EDITOR")]
    DateEditor,
    #[doc(alias = "ATSPI_ROLE_DESKTOP_ICON")]
    DesktopIcon,
    #[doc(alias = "ATSPI_ROLE_DESKTOP_FRAME")]
    DesktopFrame,
    #[doc(alias = "ATSPI_ROLE_DIAL")]
    Dial,
    #[doc(alias = "ATSPI_ROLE_DIALOG")]
    Dialog,
    #[doc(alias = "ATSPI_ROLE_DIRECTORY_PANE")]
    DirectoryPane,
    #[doc(alias = "ATSPI_ROLE_DRAWING_AREA")]
    DrawingArea,
    #[doc(alias = "ATSPI_ROLE_FILE_CHOOSER")]
    FileChooser,
    #[doc(alias = "ATSPI_ROLE_FILLER")]
    Filler,
    #[doc(alias = "ATSPI_ROLE_FOCUS_TRAVERSABLE")]
    FocusTraversable,
    #[doc(alias = "ATSPI_ROLE_FONT_CHOOSER")]
    FontChooser,
    #[doc(alias = "ATSPI_ROLE_FRAME")]
    Frame,
    #[doc(alias = "ATSPI_ROLE_GLASS_PANE")]
    GlassPane,
    #[doc(alias = "ATSPI_ROLE_HTML_CONTAINER")]
    HtmlContainer,
    #[doc(alias = "ATSPI_ROLE_ICON")]
    Icon,
    #[doc(alias = "ATSPI_ROLE_IMAGE")]
    Image,
    #[doc(alias = "ATSPI_ROLE_INTERNAL_FRAME")]
    InternalFrame,
    #[doc(alias = "ATSPI_ROLE_LABEL")]
    Label,
    #[doc(alias = "ATSPI_ROLE_LAYERED_PANE")]
    LayeredPane,
    #[doc(alias = "ATSPI_ROLE_LIST")]
    List,
    #[doc(alias = "ATSPI_ROLE_LIST_ITEM")]
    ListItem,
    #[doc(alias = "ATSPI_ROLE_MENU")]
    Menu,
    #[doc(alias = "ATSPI_ROLE_MENU_BAR")]
    MenuBar,
    #[doc(alias = "ATSPI_ROLE_MENU_ITEM")]
    MenuItem,
    #[doc(alias = "ATSPI_ROLE_OPTION_PANE")]
    OptionPane,
    #[doc(alias = "ATSPI_ROLE_PAGE_TAB")]
    PageTab,
    #[doc(alias = "ATSPI_ROLE_PAGE_TAB_LIST")]
    PageTabList,
    #[doc(alias = "ATSPI_ROLE_PANEL")]
    Panel,
    #[doc(alias = "ATSPI_ROLE_PASSWORD_TEXT")]
    PasswordText,
    #[doc(alias = "ATSPI_ROLE_POPUP_MENU")]
    PopupMenu,
    #[doc(alias = "ATSPI_ROLE_PROGRESS_BAR")]
    ProgressBar,
    #[doc(alias = "ATSPI_ROLE_PUSH_BUTTON")]
    PushButton,
    #[doc(alias = "ATSPI_ROLE_RADIO_BUTTON")]
    RadioButton,
    #[doc(alias = "ATSPI_ROLE_RADIO_MENU_ITEM")]
    RadioMenuItem,
    #[doc(alias = "ATSPI_ROLE_ROOT_PANE")]
    RootPane,
    #[doc(alias = "ATSPI_ROLE_ROW_HEADER")]
    RowHeader,
    #[doc(alias = "ATSPI_ROLE_SCROLL_BAR")]
    ScrollBar,
    #[doc(alias = "ATSPI_ROLE_SCROLL_PANE")]
    ScrollPane,
    #[doc(alias = "ATSPI_ROLE_SEPARATOR")]
    Separator,
    #[doc(alias = "ATSPI_ROLE_SLIDER")]
    Slider,
    #[doc(alias = "ATSPI_ROLE_SPIN_BUTTON")]
    SpinButton,
    #[doc(alias = "ATSPI_ROLE_SPLIT_PANE")]
    SplitPane,
    #[doc(alias = "ATSPI_ROLE_STATUS_BAR")]
    StatusBar,
    #[doc(alias = "ATSPI_ROLE_TABLE")]
    Table,
    #[doc(alias = "ATSPI_ROLE_TABLE_CELL")]
    TableCell,
    #[doc(alias = "ATSPI_ROLE_TABLE_COLUMN_HEADER")]
    TableColumnHeader,
    #[doc(alias = "ATSPI_ROLE_TABLE_ROW_HEADER")]
    TableRowHeader,
    #[doc(alias = "ATSPI_ROLE_TEAROFF_MENU_ITEM")]
    TearoffMenuItem,
    #[doc(alias = "ATSPI_ROLE_TERMINAL")]
    Terminal,
    #[doc(alias = "ATSPI_ROLE_TEXT")]
    Text,
    #[doc(alias = "ATSPI_ROLE_TOGGLE_BUTTON")]
    ToggleButton,
    #[doc(alias = "ATSPI_ROLE_TOOL_BAR")]
    ToolBar,
    #[doc(alias = "ATSPI_ROLE_TOOL_TIP")]
    ToolTip,
    #[doc(alias = "ATSPI_ROLE_TREE")]
    Tree,
    #[doc(alias = "ATSPI_ROLE_TREE_TABLE")]
    TreeTable,
    #[doc(alias = "ATSPI_ROLE_UNKNOWN")]
    Unknown,
    #[doc(alias = "ATSPI_ROLE_VIEWPORT")]
    Viewport,
    #[doc(alias = "ATSPI_ROLE_WINDOW")]
    Window,
    #[doc(alias = "ATSPI_ROLE_EXTENDED")]
    Extended,
    #[doc(alias = "ATSPI_ROLE_HEADER")]
    Header,
    #[doc(alias = "ATSPI_ROLE_FOOTER")]
    Footer,
    #[doc(alias = "ATSPI_ROLE_PARAGRAPH")]
    Paragraph,
    #[doc(alias = "ATSPI_ROLE_RULER")]
    Ruler,
    #[doc(alias = "ATSPI_ROLE_APPLICATION")]
    Application,
    #[doc(alias = "ATSPI_ROLE_AUTOCOMPLETE")]
    Autocomplete,
    #[doc(alias = "ATSPI_ROLE_EDITBAR")]
    Editbar,
    #[doc(alias = "ATSPI_ROLE_EMBEDDED")]
    Embedded,
    #[doc(alias = "ATSPI_ROLE_ENTRY")]
    Entry,
    #[doc(alias = "ATSPI_ROLE_CHART")]
    Chart,
    #[doc(alias = "ATSPI_ROLE_CAPTION")]
    Caption,
    #[doc(alias = "ATSPI_ROLE_DOCUMENT_FRAME")]
    DocumentFrame,
    #[doc(alias = "ATSPI_ROLE_HEADING")]
    Heading,
    #[doc(alias = "ATSPI_ROLE_PAGE")]
    Page,
    #[doc(alias = "ATSPI_ROLE_SECTION")]
    Section,
    #[doc(alias = "ATSPI_ROLE_REDUNDANT_OBJECT")]
    RedundantObject,
    #[doc(alias = "ATSPI_ROLE_FORM")]
    Form,
    #[doc(alias = "ATSPI_ROLE_LINK")]
    Link,
    #[doc(alias = "ATSPI_ROLE_INPUT_METHOD_WINDOW")]
    InputMethodWindow,
    #[doc(alias = "ATSPI_ROLE_TABLE_ROW")]
    TableRow,
    #[doc(alias = "ATSPI_ROLE_TREE_ITEM")]
    TreeItem,
    #[doc(alias = "ATSPI_ROLE_DOCUMENT_SPREADSHEET")]
    DocumentSpreadsheet,
    #[doc(alias = "ATSPI_ROLE_DOCUMENT_PRESENTATION")]
    DocumentPresentation,
    #[doc(alias = "ATSPI_ROLE_DOCUMENT_TEXT")]
    DocumentText,
    #[doc(alias = "ATSPI_ROLE_DOCUMENT_WEB")]
    DocumentWeb,
    #[doc(alias = "ATSPI_ROLE_DOCUMENT_EMAIL")]
    DocumentEmail,
    #[doc(alias = "ATSPI_ROLE_COMMENT")]
    Comment,
    #[doc(alias = "ATSPI_ROLE_LIST_BOX")]
    ListBox,
    #[doc(alias = "ATSPI_ROLE_GROUPING")]
    Grouping,
    #[doc(alias = "ATSPI_ROLE_IMAGE_MAP")]
    ImageMap,
    #[doc(alias = "ATSPI_ROLE_NOTIFICATION")]
    Notification,
    #[doc(alias = "ATSPI_ROLE_INFO_BAR")]
    InfoBar,
    #[doc(alias = "ATSPI_ROLE_LEVEL_BAR")]
    LevelBar,
    #[doc(alias = "ATSPI_ROLE_TITLE_BAR")]
    TitleBar,
    #[doc(alias = "ATSPI_ROLE_BLOCK_QUOTE")]
    BlockQuote,
    #[doc(alias = "ATSPI_ROLE_AUDIO")]
    Audio,
    #[doc(alias = "ATSPI_ROLE_VIDEO")]
    Video,
    #[doc(alias = "ATSPI_ROLE_DEFINITION")]
    Definition,
    #[doc(alias = "ATSPI_ROLE_ARTICLE")]
    Article,
    #[doc(alias = "ATSPI_ROLE_LANDMARK")]
    Landmark,
    #[doc(alias = "ATSPI_ROLE_LOG")]
    Log,
    #[doc(alias = "ATSPI_ROLE_MARQUEE")]
    Marquee,
    #[doc(alias = "ATSPI_ROLE_MATH")]
    Math,
    #[doc(alias = "ATSPI_ROLE_RATING")]
    Rating,
    #[doc(alias = "ATSPI_ROLE_TIMER")]
    Timer,
    #[doc(alias = "ATSPI_ROLE_STATIC")]
    Static,
    #[doc(alias = "ATSPI_ROLE_MATH_FRACTION")]
    MathFraction,
    #[doc(alias = "ATSPI_ROLE_MATH_ROOT")]
    MathRoot,
    #[doc(alias = "ATSPI_ROLE_SUBSCRIPT")]
    Subscript,
    #[doc(alias = "ATSPI_ROLE_SUPERSCRIPT")]
    Superscript,
    #[doc(alias = "ATSPI_ROLE_DESCRIPTION_LIST")]
    DescriptionList,
    #[doc(alias = "ATSPI_ROLE_DESCRIPTION_TERM")]
    DescriptionTerm,
    #[doc(alias = "ATSPI_ROLE_DESCRIPTION_VALUE")]
    DescriptionValue,
    #[doc(alias = "ATSPI_ROLE_FOOTNOTE")]
    Footnote,
    #[doc(alias = "ATSPI_ROLE_CONTENT_DELETION")]
    ContentDeletion,
    #[doc(alias = "ATSPI_ROLE_CONTENT_INSERTION")]
    ContentInsertion,
    #[doc(alias = "ATSPI_ROLE_MARK")]
    Mark,
    #[doc(alias = "ATSPI_ROLE_SUGGESTION")]
    Suggestion,
    #[doc(alias = "ATSPI_ROLE_LAST_DEFINED")]
    LastDefined,
#[doc(hidden)]
    __Unknown(i32),
}

impl Role {
    #[doc(alias = "atspi_role_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::atspi_role_get_name(self.into_glib()))
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Role::{}", match *self {
            Self::Invalid => "Invalid",
            Self::AcceleratorLabel => "AcceleratorLabel",
            Self::Alert => "Alert",
            Self::Animation => "Animation",
            Self::Arrow => "Arrow",
            Self::Calendar => "Calendar",
            Self::Canvas => "Canvas",
            Self::CheckBox => "CheckBox",
            Self::CheckMenuItem => "CheckMenuItem",
            Self::ColorChooser => "ColorChooser",
            Self::ColumnHeader => "ColumnHeader",
            Self::ComboBox => "ComboBox",
            Self::DateEditor => "DateEditor",
            Self::DesktopIcon => "DesktopIcon",
            Self::DesktopFrame => "DesktopFrame",
            Self::Dial => "Dial",
            Self::Dialog => "Dialog",
            Self::DirectoryPane => "DirectoryPane",
            Self::DrawingArea => "DrawingArea",
            Self::FileChooser => "FileChooser",
            Self::Filler => "Filler",
            Self::FocusTraversable => "FocusTraversable",
            Self::FontChooser => "FontChooser",
            Self::Frame => "Frame",
            Self::GlassPane => "GlassPane",
            Self::HtmlContainer => "HtmlContainer",
            Self::Icon => "Icon",
            Self::Image => "Image",
            Self::InternalFrame => "InternalFrame",
            Self::Label => "Label",
            Self::LayeredPane => "LayeredPane",
            Self::List => "List",
            Self::ListItem => "ListItem",
            Self::Menu => "Menu",
            Self::MenuBar => "MenuBar",
            Self::MenuItem => "MenuItem",
            Self::OptionPane => "OptionPane",
            Self::PageTab => "PageTab",
            Self::PageTabList => "PageTabList",
            Self::Panel => "Panel",
            Self::PasswordText => "PasswordText",
            Self::PopupMenu => "PopupMenu",
            Self::ProgressBar => "ProgressBar",
            Self::PushButton => "PushButton",
            Self::RadioButton => "RadioButton",
            Self::RadioMenuItem => "RadioMenuItem",
            Self::RootPane => "RootPane",
            Self::RowHeader => "RowHeader",
            Self::ScrollBar => "ScrollBar",
            Self::ScrollPane => "ScrollPane",
            Self::Separator => "Separator",
            Self::Slider => "Slider",
            Self::SpinButton => "SpinButton",
            Self::SplitPane => "SplitPane",
            Self::StatusBar => "StatusBar",
            Self::Table => "Table",
            Self::TableCell => "TableCell",
            Self::TableColumnHeader => "TableColumnHeader",
            Self::TableRowHeader => "TableRowHeader",
            Self::TearoffMenuItem => "TearoffMenuItem",
            Self::Terminal => "Terminal",
            Self::Text => "Text",
            Self::ToggleButton => "ToggleButton",
            Self::ToolBar => "ToolBar",
            Self::ToolTip => "ToolTip",
            Self::Tree => "Tree",
            Self::TreeTable => "TreeTable",
            Self::Unknown => "Unknown",
            Self::Viewport => "Viewport",
            Self::Window => "Window",
            Self::Extended => "Extended",
            Self::Header => "Header",
            Self::Footer => "Footer",
            Self::Paragraph => "Paragraph",
            Self::Ruler => "Ruler",
            Self::Application => "Application",
            Self::Autocomplete => "Autocomplete",
            Self::Editbar => "Editbar",
            Self::Embedded => "Embedded",
            Self::Entry => "Entry",
            Self::Chart => "Chart",
            Self::Caption => "Caption",
            Self::DocumentFrame => "DocumentFrame",
            Self::Heading => "Heading",
            Self::Page => "Page",
            Self::Section => "Section",
            Self::RedundantObject => "RedundantObject",
            Self::Form => "Form",
            Self::Link => "Link",
            Self::InputMethodWindow => "InputMethodWindow",
            Self::TableRow => "TableRow",
            Self::TreeItem => "TreeItem",
            Self::DocumentSpreadsheet => "DocumentSpreadsheet",
            Self::DocumentPresentation => "DocumentPresentation",
            Self::DocumentText => "DocumentText",
            Self::DocumentWeb => "DocumentWeb",
            Self::DocumentEmail => "DocumentEmail",
            Self::Comment => "Comment",
            Self::ListBox => "ListBox",
            Self::Grouping => "Grouping",
            Self::ImageMap => "ImageMap",
            Self::Notification => "Notification",
            Self::InfoBar => "InfoBar",
            Self::LevelBar => "LevelBar",
            Self::TitleBar => "TitleBar",
            Self::BlockQuote => "BlockQuote",
            Self::Audio => "Audio",
            Self::Video => "Video",
            Self::Definition => "Definition",
            Self::Article => "Article",
            Self::Landmark => "Landmark",
            Self::Log => "Log",
            Self::Marquee => "Marquee",
            Self::Math => "Math",
            Self::Rating => "Rating",
            Self::Timer => "Timer",
            Self::Static => "Static",
            Self::MathFraction => "MathFraction",
            Self::MathRoot => "MathRoot",
            Self::Subscript => "Subscript",
            Self::Superscript => "Superscript",
            Self::DescriptionList => "DescriptionList",
            Self::DescriptionTerm => "DescriptionTerm",
            Self::DescriptionValue => "DescriptionValue",
            Self::Footnote => "Footnote",
            Self::ContentDeletion => "ContentDeletion",
            Self::ContentInsertion => "ContentInsertion",
            Self::Mark => "Mark",
            Self::Suggestion => "Suggestion",
            Self::LastDefined => "LastDefined",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl IntoGlib for Role {
    type GlibType = ffi::AtspiRole;

    fn into_glib(self) -> ffi::AtspiRole {
        match self {
            Self::Invalid => ffi::ATSPI_ROLE_INVALID,
            Self::AcceleratorLabel => ffi::ATSPI_ROLE_ACCELERATOR_LABEL,
            Self::Alert => ffi::ATSPI_ROLE_ALERT,
            Self::Animation => ffi::ATSPI_ROLE_ANIMATION,
            Self::Arrow => ffi::ATSPI_ROLE_ARROW,
            Self::Calendar => ffi::ATSPI_ROLE_CALENDAR,
            Self::Canvas => ffi::ATSPI_ROLE_CANVAS,
            Self::CheckBox => ffi::ATSPI_ROLE_CHECK_BOX,
            Self::CheckMenuItem => ffi::ATSPI_ROLE_CHECK_MENU_ITEM,
            Self::ColorChooser => ffi::ATSPI_ROLE_COLOR_CHOOSER,
            Self::ColumnHeader => ffi::ATSPI_ROLE_COLUMN_HEADER,
            Self::ComboBox => ffi::ATSPI_ROLE_COMBO_BOX,
            Self::DateEditor => ffi::ATSPI_ROLE_DATE_EDITOR,
            Self::DesktopIcon => ffi::ATSPI_ROLE_DESKTOP_ICON,
            Self::DesktopFrame => ffi::ATSPI_ROLE_DESKTOP_FRAME,
            Self::Dial => ffi::ATSPI_ROLE_DIAL,
            Self::Dialog => ffi::ATSPI_ROLE_DIALOG,
            Self::DirectoryPane => ffi::ATSPI_ROLE_DIRECTORY_PANE,
            Self::DrawingArea => ffi::ATSPI_ROLE_DRAWING_AREA,
            Self::FileChooser => ffi::ATSPI_ROLE_FILE_CHOOSER,
            Self::Filler => ffi::ATSPI_ROLE_FILLER,
            Self::FocusTraversable => ffi::ATSPI_ROLE_FOCUS_TRAVERSABLE,
            Self::FontChooser => ffi::ATSPI_ROLE_FONT_CHOOSER,
            Self::Frame => ffi::ATSPI_ROLE_FRAME,
            Self::GlassPane => ffi::ATSPI_ROLE_GLASS_PANE,
            Self::HtmlContainer => ffi::ATSPI_ROLE_HTML_CONTAINER,
            Self::Icon => ffi::ATSPI_ROLE_ICON,
            Self::Image => ffi::ATSPI_ROLE_IMAGE,
            Self::InternalFrame => ffi::ATSPI_ROLE_INTERNAL_FRAME,
            Self::Label => ffi::ATSPI_ROLE_LABEL,
            Self::LayeredPane => ffi::ATSPI_ROLE_LAYERED_PANE,
            Self::List => ffi::ATSPI_ROLE_LIST,
            Self::ListItem => ffi::ATSPI_ROLE_LIST_ITEM,
            Self::Menu => ffi::ATSPI_ROLE_MENU,
            Self::MenuBar => ffi::ATSPI_ROLE_MENU_BAR,
            Self::MenuItem => ffi::ATSPI_ROLE_MENU_ITEM,
            Self::OptionPane => ffi::ATSPI_ROLE_OPTION_PANE,
            Self::PageTab => ffi::ATSPI_ROLE_PAGE_TAB,
            Self::PageTabList => ffi::ATSPI_ROLE_PAGE_TAB_LIST,
            Self::Panel => ffi::ATSPI_ROLE_PANEL,
            Self::PasswordText => ffi::ATSPI_ROLE_PASSWORD_TEXT,
            Self::PopupMenu => ffi::ATSPI_ROLE_POPUP_MENU,
            Self::ProgressBar => ffi::ATSPI_ROLE_PROGRESS_BAR,
            Self::PushButton => ffi::ATSPI_ROLE_PUSH_BUTTON,
            Self::RadioButton => ffi::ATSPI_ROLE_RADIO_BUTTON,
            Self::RadioMenuItem => ffi::ATSPI_ROLE_RADIO_MENU_ITEM,
            Self::RootPane => ffi::ATSPI_ROLE_ROOT_PANE,
            Self::RowHeader => ffi::ATSPI_ROLE_ROW_HEADER,
            Self::ScrollBar => ffi::ATSPI_ROLE_SCROLL_BAR,
            Self::ScrollPane => ffi::ATSPI_ROLE_SCROLL_PANE,
            Self::Separator => ffi::ATSPI_ROLE_SEPARATOR,
            Self::Slider => ffi::ATSPI_ROLE_SLIDER,
            Self::SpinButton => ffi::ATSPI_ROLE_SPIN_BUTTON,
            Self::SplitPane => ffi::ATSPI_ROLE_SPLIT_PANE,
            Self::StatusBar => ffi::ATSPI_ROLE_STATUS_BAR,
            Self::Table => ffi::ATSPI_ROLE_TABLE,
            Self::TableCell => ffi::ATSPI_ROLE_TABLE_CELL,
            Self::TableColumnHeader => ffi::ATSPI_ROLE_TABLE_COLUMN_HEADER,
            Self::TableRowHeader => ffi::ATSPI_ROLE_TABLE_ROW_HEADER,
            Self::TearoffMenuItem => ffi::ATSPI_ROLE_TEAROFF_MENU_ITEM,
            Self::Terminal => ffi::ATSPI_ROLE_TERMINAL,
            Self::Text => ffi::ATSPI_ROLE_TEXT,
            Self::ToggleButton => ffi::ATSPI_ROLE_TOGGLE_BUTTON,
            Self::ToolBar => ffi::ATSPI_ROLE_TOOL_BAR,
            Self::ToolTip => ffi::ATSPI_ROLE_TOOL_TIP,
            Self::Tree => ffi::ATSPI_ROLE_TREE,
            Self::TreeTable => ffi::ATSPI_ROLE_TREE_TABLE,
            Self::Unknown => ffi::ATSPI_ROLE_UNKNOWN,
            Self::Viewport => ffi::ATSPI_ROLE_VIEWPORT,
            Self::Window => ffi::ATSPI_ROLE_WINDOW,
            Self::Extended => ffi::ATSPI_ROLE_EXTENDED,
            Self::Header => ffi::ATSPI_ROLE_HEADER,
            Self::Footer => ffi::ATSPI_ROLE_FOOTER,
            Self::Paragraph => ffi::ATSPI_ROLE_PARAGRAPH,
            Self::Ruler => ffi::ATSPI_ROLE_RULER,
            Self::Application => ffi::ATSPI_ROLE_APPLICATION,
            Self::Autocomplete => ffi::ATSPI_ROLE_AUTOCOMPLETE,
            Self::Editbar => ffi::ATSPI_ROLE_EDITBAR,
            Self::Embedded => ffi::ATSPI_ROLE_EMBEDDED,
            Self::Entry => ffi::ATSPI_ROLE_ENTRY,
            Self::Chart => ffi::ATSPI_ROLE_CHART,
            Self::Caption => ffi::ATSPI_ROLE_CAPTION,
            Self::DocumentFrame => ffi::ATSPI_ROLE_DOCUMENT_FRAME,
            Self::Heading => ffi::ATSPI_ROLE_HEADING,
            Self::Page => ffi::ATSPI_ROLE_PAGE,
            Self::Section => ffi::ATSPI_ROLE_SECTION,
            Self::RedundantObject => ffi::ATSPI_ROLE_REDUNDANT_OBJECT,
            Self::Form => ffi::ATSPI_ROLE_FORM,
            Self::Link => ffi::ATSPI_ROLE_LINK,
            Self::InputMethodWindow => ffi::ATSPI_ROLE_INPUT_METHOD_WINDOW,
            Self::TableRow => ffi::ATSPI_ROLE_TABLE_ROW,
            Self::TreeItem => ffi::ATSPI_ROLE_TREE_ITEM,
            Self::DocumentSpreadsheet => ffi::ATSPI_ROLE_DOCUMENT_SPREADSHEET,
            Self::DocumentPresentation => ffi::ATSPI_ROLE_DOCUMENT_PRESENTATION,
            Self::DocumentText => ffi::ATSPI_ROLE_DOCUMENT_TEXT,
            Self::DocumentWeb => ffi::ATSPI_ROLE_DOCUMENT_WEB,
            Self::DocumentEmail => ffi::ATSPI_ROLE_DOCUMENT_EMAIL,
            Self::Comment => ffi::ATSPI_ROLE_COMMENT,
            Self::ListBox => ffi::ATSPI_ROLE_LIST_BOX,
            Self::Grouping => ffi::ATSPI_ROLE_GROUPING,
            Self::ImageMap => ffi::ATSPI_ROLE_IMAGE_MAP,
            Self::Notification => ffi::ATSPI_ROLE_NOTIFICATION,
            Self::InfoBar => ffi::ATSPI_ROLE_INFO_BAR,
            Self::LevelBar => ffi::ATSPI_ROLE_LEVEL_BAR,
            Self::TitleBar => ffi::ATSPI_ROLE_TITLE_BAR,
            Self::BlockQuote => ffi::ATSPI_ROLE_BLOCK_QUOTE,
            Self::Audio => ffi::ATSPI_ROLE_AUDIO,
            Self::Video => ffi::ATSPI_ROLE_VIDEO,
            Self::Definition => ffi::ATSPI_ROLE_DEFINITION,
            Self::Article => ffi::ATSPI_ROLE_ARTICLE,
            Self::Landmark => ffi::ATSPI_ROLE_LANDMARK,
            Self::Log => ffi::ATSPI_ROLE_LOG,
            Self::Marquee => ffi::ATSPI_ROLE_MARQUEE,
            Self::Math => ffi::ATSPI_ROLE_MATH,
            Self::Rating => ffi::ATSPI_ROLE_RATING,
            Self::Timer => ffi::ATSPI_ROLE_TIMER,
            Self::Static => ffi::ATSPI_ROLE_STATIC,
            Self::MathFraction => ffi::ATSPI_ROLE_MATH_FRACTION,
            Self::MathRoot => ffi::ATSPI_ROLE_MATH_ROOT,
            Self::Subscript => ffi::ATSPI_ROLE_SUBSCRIPT,
            Self::Superscript => ffi::ATSPI_ROLE_SUPERSCRIPT,
            Self::DescriptionList => ffi::ATSPI_ROLE_DESCRIPTION_LIST,
            Self::DescriptionTerm => ffi::ATSPI_ROLE_DESCRIPTION_TERM,
            Self::DescriptionValue => ffi::ATSPI_ROLE_DESCRIPTION_VALUE,
            Self::Footnote => ffi::ATSPI_ROLE_FOOTNOTE,
            Self::ContentDeletion => ffi::ATSPI_ROLE_CONTENT_DELETION,
            Self::ContentInsertion => ffi::ATSPI_ROLE_CONTENT_INSERTION,
            Self::Mark => ffi::ATSPI_ROLE_MARK,
            Self::Suggestion => ffi::ATSPI_ROLE_SUGGESTION,
            Self::LastDefined => ffi::ATSPI_ROLE_LAST_DEFINED,
            Self::__Unknown(value) => value,
}
    }
}

#[doc(hidden)]
impl FromGlib<ffi::AtspiRole> for Role {
    unsafe fn from_glib(value: ffi::AtspiRole) -> Self {
        match value {
            ffi::ATSPI_ROLE_INVALID => Self::Invalid,
            ffi::ATSPI_ROLE_ACCELERATOR_LABEL => Self::AcceleratorLabel,
            ffi::ATSPI_ROLE_ALERT => Self::Alert,
            ffi::ATSPI_ROLE_ANIMATION => Self::Animation,
            ffi::ATSPI_ROLE_ARROW => Self::Arrow,
            ffi::ATSPI_ROLE_CALENDAR => Self::Calendar,
            ffi::ATSPI_ROLE_CANVAS => Self::Canvas,
            ffi::ATSPI_ROLE_CHECK_BOX => Self::CheckBox,
            ffi::ATSPI_ROLE_CHECK_MENU_ITEM => Self::CheckMenuItem,
            ffi::ATSPI_ROLE_COLOR_CHOOSER => Self::ColorChooser,
            ffi::ATSPI_ROLE_COLUMN_HEADER => Self::ColumnHeader,
            ffi::ATSPI_ROLE_COMBO_BOX => Self::ComboBox,
            ffi::ATSPI_ROLE_DATE_EDITOR => Self::DateEditor,
            ffi::ATSPI_ROLE_DESKTOP_ICON => Self::DesktopIcon,
            ffi::ATSPI_ROLE_DESKTOP_FRAME => Self::DesktopFrame,
            ffi::ATSPI_ROLE_DIAL => Self::Dial,
            ffi::ATSPI_ROLE_DIALOG => Self::Dialog,
            ffi::ATSPI_ROLE_DIRECTORY_PANE => Self::DirectoryPane,
            ffi::ATSPI_ROLE_DRAWING_AREA => Self::DrawingArea,
            ffi::ATSPI_ROLE_FILE_CHOOSER => Self::FileChooser,
            ffi::ATSPI_ROLE_FILLER => Self::Filler,
            ffi::ATSPI_ROLE_FOCUS_TRAVERSABLE => Self::FocusTraversable,
            ffi::ATSPI_ROLE_FONT_CHOOSER => Self::FontChooser,
            ffi::ATSPI_ROLE_FRAME => Self::Frame,
            ffi::ATSPI_ROLE_GLASS_PANE => Self::GlassPane,
            ffi::ATSPI_ROLE_HTML_CONTAINER => Self::HtmlContainer,
            ffi::ATSPI_ROLE_ICON => Self::Icon,
            ffi::ATSPI_ROLE_IMAGE => Self::Image,
            ffi::ATSPI_ROLE_INTERNAL_FRAME => Self::InternalFrame,
            ffi::ATSPI_ROLE_LABEL => Self::Label,
            ffi::ATSPI_ROLE_LAYERED_PANE => Self::LayeredPane,
            ffi::ATSPI_ROLE_LIST => Self::List,
            ffi::ATSPI_ROLE_LIST_ITEM => Self::ListItem,
            ffi::ATSPI_ROLE_MENU => Self::Menu,
            ffi::ATSPI_ROLE_MENU_BAR => Self::MenuBar,
            ffi::ATSPI_ROLE_MENU_ITEM => Self::MenuItem,
            ffi::ATSPI_ROLE_OPTION_PANE => Self::OptionPane,
            ffi::ATSPI_ROLE_PAGE_TAB => Self::PageTab,
            ffi::ATSPI_ROLE_PAGE_TAB_LIST => Self::PageTabList,
            ffi::ATSPI_ROLE_PANEL => Self::Panel,
            ffi::ATSPI_ROLE_PASSWORD_TEXT => Self::PasswordText,
            ffi::ATSPI_ROLE_POPUP_MENU => Self::PopupMenu,
            ffi::ATSPI_ROLE_PROGRESS_BAR => Self::ProgressBar,
            ffi::ATSPI_ROLE_PUSH_BUTTON => Self::PushButton,
            ffi::ATSPI_ROLE_RADIO_BUTTON => Self::RadioButton,
            ffi::ATSPI_ROLE_RADIO_MENU_ITEM => Self::RadioMenuItem,
            ffi::ATSPI_ROLE_ROOT_PANE => Self::RootPane,
            ffi::ATSPI_ROLE_ROW_HEADER => Self::RowHeader,
            ffi::ATSPI_ROLE_SCROLL_BAR => Self::ScrollBar,
            ffi::ATSPI_ROLE_SCROLL_PANE => Self::ScrollPane,
            ffi::ATSPI_ROLE_SEPARATOR => Self::Separator,
            ffi::ATSPI_ROLE_SLIDER => Self::Slider,
            ffi::ATSPI_ROLE_SPIN_BUTTON => Self::SpinButton,
            ffi::ATSPI_ROLE_SPLIT_PANE => Self::SplitPane,
            ffi::ATSPI_ROLE_STATUS_BAR => Self::StatusBar,
            ffi::ATSPI_ROLE_TABLE => Self::Table,
            ffi::ATSPI_ROLE_TABLE_CELL => Self::TableCell,
            ffi::ATSPI_ROLE_TABLE_COLUMN_HEADER => Self::TableColumnHeader,
            ffi::ATSPI_ROLE_TABLE_ROW_HEADER => Self::TableRowHeader,
            ffi::ATSPI_ROLE_TEAROFF_MENU_ITEM => Self::TearoffMenuItem,
            ffi::ATSPI_ROLE_TERMINAL => Self::Terminal,
            ffi::ATSPI_ROLE_TEXT => Self::Text,
            ffi::ATSPI_ROLE_TOGGLE_BUTTON => Self::ToggleButton,
            ffi::ATSPI_ROLE_TOOL_BAR => Self::ToolBar,
            ffi::ATSPI_ROLE_TOOL_TIP => Self::ToolTip,
            ffi::ATSPI_ROLE_TREE => Self::Tree,
            ffi::ATSPI_ROLE_TREE_TABLE => Self::TreeTable,
            ffi::ATSPI_ROLE_UNKNOWN => Self::Unknown,
            ffi::ATSPI_ROLE_VIEWPORT => Self::Viewport,
            ffi::ATSPI_ROLE_WINDOW => Self::Window,
            ffi::ATSPI_ROLE_EXTENDED => Self::Extended,
            ffi::ATSPI_ROLE_HEADER => Self::Header,
            ffi::ATSPI_ROLE_FOOTER => Self::Footer,
            ffi::ATSPI_ROLE_PARAGRAPH => Self::Paragraph,
            ffi::ATSPI_ROLE_RULER => Self::Ruler,
            ffi::ATSPI_ROLE_APPLICATION => Self::Application,
            ffi::ATSPI_ROLE_AUTOCOMPLETE => Self::Autocomplete,
            ffi::ATSPI_ROLE_EDITBAR => Self::Editbar,
            ffi::ATSPI_ROLE_EMBEDDED => Self::Embedded,
            ffi::ATSPI_ROLE_ENTRY => Self::Entry,
            ffi::ATSPI_ROLE_CHART => Self::Chart,
            ffi::ATSPI_ROLE_CAPTION => Self::Caption,
            ffi::ATSPI_ROLE_DOCUMENT_FRAME => Self::DocumentFrame,
            ffi::ATSPI_ROLE_HEADING => Self::Heading,
            ffi::ATSPI_ROLE_PAGE => Self::Page,
            ffi::ATSPI_ROLE_SECTION => Self::Section,
            ffi::ATSPI_ROLE_REDUNDANT_OBJECT => Self::RedundantObject,
            ffi::ATSPI_ROLE_FORM => Self::Form,
            ffi::ATSPI_ROLE_LINK => Self::Link,
            ffi::ATSPI_ROLE_INPUT_METHOD_WINDOW => Self::InputMethodWindow,
            ffi::ATSPI_ROLE_TABLE_ROW => Self::TableRow,
            ffi::ATSPI_ROLE_TREE_ITEM => Self::TreeItem,
            ffi::ATSPI_ROLE_DOCUMENT_SPREADSHEET => Self::DocumentSpreadsheet,
            ffi::ATSPI_ROLE_DOCUMENT_PRESENTATION => Self::DocumentPresentation,
            ffi::ATSPI_ROLE_DOCUMENT_TEXT => Self::DocumentText,
            ffi::ATSPI_ROLE_DOCUMENT_WEB => Self::DocumentWeb,
            ffi::ATSPI_ROLE_DOCUMENT_EMAIL => Self::DocumentEmail,
            ffi::ATSPI_ROLE_COMMENT => Self::Comment,
            ffi::ATSPI_ROLE_LIST_BOX => Self::ListBox,
            ffi::ATSPI_ROLE_GROUPING => Self::Grouping,
            ffi::ATSPI_ROLE_IMAGE_MAP => Self::ImageMap,
            ffi::ATSPI_ROLE_NOTIFICATION => Self::Notification,
            ffi::ATSPI_ROLE_INFO_BAR => Self::InfoBar,
            ffi::ATSPI_ROLE_LEVEL_BAR => Self::LevelBar,
            ffi::ATSPI_ROLE_TITLE_BAR => Self::TitleBar,
            ffi::ATSPI_ROLE_BLOCK_QUOTE => Self::BlockQuote,
            ffi::ATSPI_ROLE_AUDIO => Self::Audio,
            ffi::ATSPI_ROLE_VIDEO => Self::Video,
            ffi::ATSPI_ROLE_DEFINITION => Self::Definition,
            ffi::ATSPI_ROLE_ARTICLE => Self::Article,
            ffi::ATSPI_ROLE_LANDMARK => Self::Landmark,
            ffi::ATSPI_ROLE_LOG => Self::Log,
            ffi::ATSPI_ROLE_MARQUEE => Self::Marquee,
            ffi::ATSPI_ROLE_MATH => Self::Math,
            ffi::ATSPI_ROLE_RATING => Self::Rating,
            ffi::ATSPI_ROLE_TIMER => Self::Timer,
            ffi::ATSPI_ROLE_STATIC => Self::Static,
            ffi::ATSPI_ROLE_MATH_FRACTION => Self::MathFraction,
            ffi::ATSPI_ROLE_MATH_ROOT => Self::MathRoot,
            ffi::ATSPI_ROLE_SUBSCRIPT => Self::Subscript,
            ffi::ATSPI_ROLE_SUPERSCRIPT => Self::Superscript,
            ffi::ATSPI_ROLE_DESCRIPTION_LIST => Self::DescriptionList,
            ffi::ATSPI_ROLE_DESCRIPTION_TERM => Self::DescriptionTerm,
            ffi::ATSPI_ROLE_DESCRIPTION_VALUE => Self::DescriptionValue,
            ffi::ATSPI_ROLE_FOOTNOTE => Self::Footnote,
            ffi::ATSPI_ROLE_CONTENT_DELETION => Self::ContentDeletion,
            ffi::ATSPI_ROLE_CONTENT_INSERTION => Self::ContentInsertion,
            ffi::ATSPI_ROLE_MARK => Self::Mark,
            ffi::ATSPI_ROLE_SUGGESTION => Self::Suggestion,
            ffi::ATSPI_ROLE_LAST_DEFINED => Self::LastDefined,
            value => Self::__Unknown(value),
}
    }
}

impl StaticType for Role {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::atspi_role_get_type()) }
    }
}

impl glib::value::ValueType for Role {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for Role {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for Role {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

