# The Notorio syntax specification

## Overview

Notorio is a markup(ish) language which facilitates making clean, readable notes.  
It is based on a hierarchical structure. The author, title and related data are written at the top,
with any other elements being treated as Items.

## Syntax elements

### Items
An Item is defined within the root as any string without any indentation in front of it.

The following are valid Item definitions:

```
MyItem
  some interesting information here

My Second Item
  spaces are completely fine

a third item
  lowercase is allowed too

4th_Item
  so are numbers and special characters
  except ( and [
  those indicate the beginning of a data block and region respectively
```

### Data Blocks
A Data Block gives extra information about any element, including the root element. You can define custom pieces of information, but all elements implement the following:

 - `info`: A simple bit of extra information. Appears in grey and in a smaller, italic font.
 - `importance`: one of 1, 2, 3, 4; alters the colour, border thickness and font boldness where applicable.

Furthermore, the root element also implements the `author`, `title` and `copyright` attributes, though none are required.  
Data Blocks are written as a pair of parentheses with attribute-value pairs separated by semicolons.

Examples:
```
TheSpanishInquisition
  Nobody expects this paragraph! (info: This is a Monty Python reference.; importance: 3)
  Comfy chairs are no match for me! (info: Just watch Flying Circus already; importance: 1)
```

### Paragraphs
An Item is populated by paragraphs. Any text that is indented by a single tab and is part of an Item gets treated as a paragraph.

Examples:

```
AllYourItemAreBelongToUs
  this gets treated as a paragraph
  2 so does this
  "This will appear as text with quotation marks around it"
    And all of these bits of text will get treated
    as a list of blocks of text, because there's more than one

```

### Explanations
An Explanation is a special Paragraph that's used to explain something, obviously.
You can explicitly define one by using the "=" syntax, like so:
```
This is an Item
  Here's a complex concept
  = And this is an explanation
```

However, a Paragraph with only one Paragraph without any subelements as its only subelement is automatically inferred to be an explanation:
```
OhJeezAnotherItem
  And by golly, another bit of text that needs explaining!
    Luckily, it's easy to do like this
```

### Arrow shorthand
You can render directional arrows using the arrow shorthand like this: `!up`, `!dwn`, etcetera.  
A list of possible arrows can be found below:

| Arrow type        | Shorthand | Unicode codepoint | Unicode |
|-------------------|-----------|-------------------|---------|
| Single left       | !lft      | U+2190            | ←       |
| Single right      | !rgt      | U+2192            | →       |
| Single down       | !dwn      | U+2193            | ↓       |
| Single up         | !up       | U+2191            | ↑       |
| Single left-right | !lftrgt   | U+2194            | ↔       |
| Single up-down    | !updwn    | U+2195            | ↕       |
| Double left       | !Lft      | U+21D0            | ⇐       |
| Double right      | !Rgt      | U+21D2            | ⇒       |
| Double up         | !Up       | U+21D1            | ⇑       |
| Double down       | !Dwn      | U+21D3            | ⇓       |
| Double left-right | !Lftrgt   | U+21D4            | ⇔       |
| Double up-down    | !Updwn    | U+21D5            | ⇕       |

There are over 600 arrows in Unicode. I am not implementing them all. Make a bloody pull request.

### Results
A Result is like an Explanation, but it itself can have multiple children and successors (other Results that follow it).  
You can define a Result with the `->` syntax like this:
```
This is the main Paragraph
  Hey look, an event
  ->This is a result of that event
```

Chaining Results is useful when a series of events occur after a consequential event; like this:
```
My Parents
  My parents have a child; me
  ->I grow up
  ->I learn to code
  ->I write Notorio
  My parents have another child; my brother
    He's there too, I guess
```


### Regions
A Region is a bit special, in that it itself doesn't have a graphical representation. Instead, it defines an area in which special formatting rules apply to the Paragraphs below it. An example is the `list` region:
```
ThisItemContainsInformation
  [list]
    Now, all of these Paragraphs will have the same indentation.
    They've also got bullet points too
    What's more, is that if a Paragraph has children, like this...
      All of the children will have
      A different kind of bullet point
      Cool, right?
  ]list[
```

As you can see, a Region starts with "correct-facing" curly brackets and ends with "opposite-facing ones".  
One of the strongest features of Regions are the fact that you can supply a data argument. Say, for example that you have a list of dates on which some important historical events happened. You want to display them nicely but you don't have dates for all of them, so you use a list like this:
```
The French Revolution
  Dissolution of the Ancien Régime
    Some important historical events:
    [list date]
      04-08-1789 Abolition of the privileges !rgt Removal of the classes system
      26-08-1789 Declaration of the Rights of the People and of the Citizens
      / Judicial equality
      / All church goods get transferred to the State
    ]list[
    ->State debt problem got solved
```

The currently supported Regions are:
 - list
   - Arguments: date, datetime, time, generic, *none* (default)
 - order
   - Arguments: roman, Roman, *arabic* (default)
 - aligned
   - Arguments: left, *center* (default), right

## Styling a note
A Notorio style file (extension .nts) defines all the style elements of a note. This includes fonts, importance colours, look of regions and info text, border thickness and much more. .nts files look pretty similar to CSS, except without the semicolons and curly braces. An example styling file looks like this:

```
Item
	colour: l(240)
	borderColour: l(160)
	borderThickness: 2
	font: "Source Sans Pro", sans-serif

	importanceColours: {
		1: rgb(147, 219, 166)
		2: rgb(219, 213, 147)
		3: rgb(227, 174, 120)
		4: rgb(227, 129, 120)
	}


Explanation
	fontSize: [inherit] * 0.75
	fontColour: l(50)
```
## Building a note
Run `notorio -i <YOUR SOURCE FILE HERE>`. The default name is just the name of the source file.

### Export quality
Using the `-q` flag, you can choose an export quality. This is one of `low`, `med`, `high` or `very_high`.

### Styling
You can provide a .nts file with the `-s <STYLE FILE>` option.
