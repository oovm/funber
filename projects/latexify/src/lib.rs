mod for_std;
mod for_3rd;


pub trait Latexify {
    fn latexify(&mut self, f: std::fmt::Formatter) -> std::fmt::Result;
}
