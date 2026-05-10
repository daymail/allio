use lazy_static::lazy_static;

pub struct IconSet {
    pub palette: &'static str,
    pub star: &'static str,
    pub smooth_star: &'static str,
    pub hollow_star: &'static str,
    pub badge_star: &'static str,
    pub triangle_right: &'static str,
    pub triangle_down: &'static str,
    pub circle: &'static str,
    pub square: &'static str,
    pub smooth_square: &'static str,
    pub check: &'static str,
    pub warning: &'static str,
    pub error: &'static str,
}

lazy_static! {
    pub static ref ICONS: IconSet = IconSet {
        palette: "",
        star: "󰓎",
        smooth_star: "",
        hollow_star: "",
        badge_star: "󰓏",
        triangle_right: "󰁔",
        triangle_down: "󰁕",
        circle: "󰀀",
        square: "󰝤",
        smooth_square: "",
        check: "󰄬",
        warning: "󰀦",
        error: "󰅚",
    };
}
