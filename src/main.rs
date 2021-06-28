// NOTORIO SOURCE CODE
// Author: Simeon Duwel
// This is most likely very bad, very inefficient code.
// Please, by all means, open a pull request if you want to contribute to a poor man's software
// projects. I'm not a good Rust dev, nor am I experienced. Ah well.

use termion::color;
use structopt::StructOpt;

struct Colour {
	r: u8,
	g: u8,
	b: u8
}

struct StyleOptions {
	fillColour: Colour,

	borderColour: Colour,
	borderThickness: u8,

	font: Vec<String>, //this is a vector, to provide fallback fonts
	fontSize: f32,     //float because the size could be, say, 11.5
	fontColour: Colour,

	importanceColours: Vec<Colour>,
}

struct Expandable {
	x: u32,
	y: u32,
	w: u32,
	h: u32
}

enum ElementType {
	Item,
	Paragraph,
	Result,
	ChainResult,
	List,
	TextBlock
}

struct Element {
	type: ElementType,
	child: Option<Element>
}

fn main() {
    println!("Hello, world!");
}
