use std::ops::{Add, AddAssign};

use lock::Ansii;

const BOLD_CODE : &str = "\x1b[1m";
const FAINT_CODE : &str = "\x1b[2m";
const ITALIC_CODE : &str = "\x1b[3m";
const UNDERLINE_CODE : &str = "\x1b[4m";
const INVERSE_CODE : &str = "\x1b[7m";
const CONCEAL_CODE : &str = "\x1b[8m";
const RESET_CODE : &str = "\x1b[0m";

pub trait Coloured : lock::Ansii {
    type Output;
    fn style(self, style: Style) -> <Self as Coloured>::Output;
    fn colour(self, colour: Colour) -> <Self as Coloured>::Output;
    fn background_colour(self, colour: Colour) -> <Self as Coloured>::Output;


    fn bold(self) -> <Self as Coloured>::Output where Self: Sized  {
        self.style(Style::Bold)
    }
    fn faint(self) -> <Self as Coloured>::Output where Self: Sized  {
        self.style(Style::Faint)
    }
    fn italic(self) -> <Self as Coloured>::Output where Self: Sized  {
        self.style(Style::Italic)
    }
    fn underline(self) -> <Self as Coloured>::Output where Self: Sized  {
        self.style(Style::Underline)
    }
    fn inverse_colour(self) -> <Self as Coloured>::Output where Self: Sized  {
        self.style(Style::InverseColoured)
    }
    fn conceal(self) -> <Self as Coloured>::Output where Self: Sized  {
        self.style(Style::Conceal)
    }

    fn black(self) -> <Self as Coloured>::Output where Self: Sized {
        self.colour(Colour::BLACK)
    }
    fn background_black(self) -> <Self as Coloured>::Output where Self: Sized {
        self.background_colour(Colour::BLACK)
    }
    fn red(self) -> <Self as Coloured>::Output where Self: Sized {
        self.colour(Colour::RED)
    }

    fn background_red(self) -> <Self as Coloured>::Output where Self: Sized {
        self.background_colour(Colour::RED)
    }
    fn green(self) -> <Self as Coloured>::Output where Self: Sized {
        self.colour(Colour::GREEN)
    }

    fn background_green(self) -> <Self as Coloured>::Output where Self: Sized {
        self.background_colour(Colour::GREEN)
    }
    fn yellow(self) -> <Self as Coloured>::Output where Self: Sized {
        self.colour(Colour::YELLOW)
    }

    fn background_yellow(self) -> <Self as Coloured>::Output where Self: Sized {
        self.background_colour(Colour::YELLOW)
    }
    fn blue(self) -> <Self as Coloured>::Output where Self: Sized {
        self.colour(Colour::BLUE)
    }

    fn background_blue(self) -> <Self as Coloured>::Output where Self: Sized {
        self.background_colour(Colour::BLUE)
    }
    fn magenta(self) -> <Self as Coloured>::Output where Self: Sized {
        self.colour(Colour::MAGENTA)
    }

    fn background_magenta(self) -> <Self as Coloured>::Output where Self: Sized {
        self.background_colour(Colour::MAGENTA)
    }
    fn cyan(self) -> <Self as Coloured>::Output where Self: Sized {
        self.colour(Colour::CYAN)
    }

    fn background_cyan(self) -> <Self as Coloured>::Output where Self: Sized {
        self.background_colour(Colour::CYAN)
    }
    fn grey(self) -> <Self as Coloured>::Output where Self: Sized {
        self.colour(Colour::GREY)
    }

    fn background_grey(self) -> <Self as Coloured>::Output where Self: Sized {
        self.background_colour(Colour::GREY)
    }
}

pub enum Style {
    Bold,
    Faint,
    Italic,
    Underline,
    InverseColoured,
    Conceal,
    Colour(Colour),
    Background(Colour),
    Gradient(Colour, Colour),
}

mod lock {
    pub trait Ansii {
        type Output;
        fn add_ansii(self, ansii: String) -> Self::Output;
    }
}

pub enum Colour {
    RGB(u8, u8, u8),
}

impl Colour {
    pub const RED : Colour = Colour::RGB(255, 0, 0);
    pub const BLUE : Colour = Colour::RGB(0, 0, 255);
    pub const GREEN : Colour = Colour::RGB(0, 255, 0);
    pub const YELLOW : Colour = Colour::RGB(255, 255, 0);
    pub const PURPLE : Colour = Colour::RGB(255, 0, 255);
    pub const CYAN : Colour = Colour::RGB(0, 255, 255);
    pub const ORANGE : Colour = Colour::RGB(255, 165, 0);
    pub const PINK : Colour = Colour::RGB(255, 192, 203);
    pub const BLACK : Colour = Colour::RGB(0, 0, 0);
    pub const WHITE : Colour = Colour::RGB(255, 255, 255);
    pub const MAGENTA : Colour = Colour::RGB(255, 0, 255);
    pub const BROWN : Colour = Colour::RGB(165, 42, 42);
    pub const GREY : Colour = Colour::RGB(128, 128, 128);
}

impl Colour {
    fn as_ansi(&self) -> String {
        match self {
            Colour::RGB(r, g, b) => format!("{}", rgb_to_ansi256(*r, *g, *b)),
        }
    }

    fn interpolate(&self, other: &Colour, factor: f64) -> Colour {
        match (self, other) {
            (Colour::RGB(r1, g1, b1), Colour::RGB(r2, g2, b2)) => {
                let r = (*r1 as f64 + (*r2 as f64 - *r1 as f64) * factor) as u8;
                let g = (*g1 as f64 + (*g2 as f64 - *g1 as f64) * factor) as u8;
                let b = (*b1 as f64 + (*b2 as f64 - *b1 as f64) * factor) as u8;
                Colour::RGB(r, g, b)
            },
        }
    }
}

impl Coloured for String {
    type Output = String;
    fn style(self, style: Style) -> <Self as Coloured>::Output {
        match style {
            Style::Bold => self.add_ansii(BOLD_CODE.to_string()),
            Style::Faint => self.add_ansii(FAINT_CODE.to_string()),
            Style::Italic => self.add_ansii(ITALIC_CODE.to_string()),
            Style::Underline => self.add_ansii(UNDERLINE_CODE.to_string()),
            Style::InverseColoured => self.add_ansii(INVERSE_CODE.to_string()),
            Style::Conceal => self.add_ansii(CONCEAL_CODE.to_string()),
            Style::Colour(colour)=> self.add_ansii(format!("\x1b[38;5;{}m", colour.as_ansi())),
            Style::Background(colour)=> self.add_ansii(format!("\x1b[48;5;{}m", colour.as_ansi())),
            Style::Gradient(start, end) => {
                let mut final_string = String::new();
                for (index, chr) in self.chars().enumerate() {
                    final_string = format!("{final_string}\x1b[38;5;{}m{chr}", start.interpolate(&end, index as f64 / self.len() as f64).as_ansi());
                }
                final_string.push_str(RESET_CODE);
                final_string
            },
        }
    }

    fn colour(self, colour: Colour) -> <Self as Coloured>::Output {
        self.style(Style::Colour(colour))
    }

    fn background_colour(self, colour: Colour) -> <Self as Coloured>::Output {
        self.style(Style::Background(colour))
    }
}

impl Coloured for &str {
    type Output = String;
    fn style(self, style: Style) -> <Self as Coloured>::Output {
        self.to_string().style(style)
    }

    fn colour(self, colour: Colour) -> <Self as Coloured>::Output {
        self.style(Style::Colour(colour))
    }

    fn background_colour(self, colour: Colour) -> <Self as Coloured>::Output {
        self.style(Style::Background(colour))
    }
}

impl lock::Ansii for String {
    type Output = String;
    fn add_ansii(mut self, ansii: String) -> Self::Output {
        if !self.ends_with(RESET_CODE) {
            self.add_assign(RESET_CODE)
        }
        ansii.add(&self)
    }
}

impl lock::Ansii for &str {
    type Output = String;
    fn add_ansii(self, ansii: String) -> Self::Output {
        self.to_string().add_ansii(ansii)
    }
}


fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    if r == g && g == b {
        if r < 8 {
            return 16;
        }
        if r > 248 {
            return 231;
        }
        return (((r - 8) as f32 / 247.0) * 24.0 + 232.0).round() as u8;
    }
    let ansi = 16
        + (36 * (r as f32 / 255.0 * 5.0).round() as u8)
        + (6 * (g as f32 / 255.0 * 5.0).round() as u8)
        + (b as f32 / 255.0 * 5.0).round() as u8;
    ansi
}