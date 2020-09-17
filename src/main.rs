#![allow(unused)]
trait Widget {}

trait Component {
  fn run(&mut self);
  fn get_base(&mut self) -> &mut Base;
}
struct Base {
  x: f32,
  y: f32,
  child: Option<Box::<dyn Component>>,
  children: Option<Vec<Box::<dyn Component>>>,
}

impl Base {
  pub fn default() -> Self {
    Self {
      x: 0.0,
      y: 0.0,
      child: None,
      children: None,
    }
  }

  pub fn with_child(child: Box::<dyn Component>) -> Base {
    let mut base = Base::default();
    base.child = Some(child);
    base
  }

  pub fn with_children(children: Vec<Box::<dyn Component>>) -> Base {
    let mut base = Base::default();
    base.children = Some(children);
    base
  }
}

trait BaseComp {
  fn get_pos(&self) -> &Base;
  fn set_pos(&mut self, x: f32, y: f32);
  fn get_child(&mut self) -> &mut Option<Box::<dyn Component>>;
  fn get_children(&mut self) -> &mut Option<Vec<Box::<dyn Component>>>;
}

impl BaseComp for Base {
  fn get_pos(&self) -> &Base {
    println!("My position is: {}, {}", self.x, self.y);
    &self
  }

  fn set_pos(&mut self, x: f32, y: f32) {
    self.x = x;
    self.y = y;
  }

  fn get_child(&mut self) -> &mut Option<Box::<dyn Component>> {
    &mut self.child
  }

  fn get_children(&mut self) -> &mut Option<Vec<Box::<dyn Component>>> {
    &mut self.children
  }
}

struct Column {
  base: Base,
}

impl Column {
  pub fn new(children: Vec<Box::<dyn Component>>) -> Box::<Self> {
    Box::new(Self{
      base: Base::with_children(children),
    })
  }
} 

impl Component for Column {
  fn run(&mut self) {
    println!("column running!")
  }

  fn get_base(&mut self) -> &mut Base {
      &mut self.base
  }
}
struct Button {
  pub base: Base,
}

impl Button {
  pub fn new() -> Box::<dyn Component> {
    Box::new(Button{
      base: Base::default()
    })
  }

}

impl Component for Button {
  fn run(&mut self) {
    println!("i'm a button!")
  }
  fn get_base(&mut self) -> &mut Base {
    &mut self.base
  }

}

struct App {
  pub body: Box::<dyn Component>,
  pub base: Base,
}

impl App {
  fn new(body: Box<dyn Component>) -> Self {
    Self{
      body,
      base: Base::default()
    }
  }

  pub fn run(&mut self) {
    App::parse_child(&mut self.body)
  }

  fn parse_child(child: &mut Box::<dyn Component>) {

  }

}

struct TextType {
  text: String
}

enum WgtType {
  Text(TextType),
  Column,
}

enum ChType<'a> {
  Child(Box::<BaseWidget<'a>>),
  Children(Vec<Box<&'a mut BaseWidget<'a>>>),
  None,
}

struct BaseWidget<'a> {
  parent: Option<Box::<BaseWidget<'a>>>,
  wgt_type: WgtType,
  ch_type: ChType<'a>,
  top: f32,
  left: f32,
  width: f32,
  height: f32,
}

impl<'a> BaseWidget<'a> {
  pub fn build(&mut self) {
    let im_type = match self.wgt_type {
      WgtType::Column => "column",
      WgtType::Text(_) => "text",
    };
    println!("I'm a {} and my postion is x:{:.1} y:{:.1}", im_type, self.top, self.left);

    match &self.wgt_type {
      WgtType::Text(t) => println!("Draw text to screen!"),
      WgtType::Column => {
        match &mut self.ch_type {
          ChType::Children(children) => {
            println!("Draw column");
            for child in children {
              child.set_position(1.0, 1.0);
              child.build()
            }
          },
          _ => {},
        }
      }
    }
  }

  pub fn set_position(&mut self, top: f32, left: f32) {
    self.top = top;
  }
}

fn text<'a>(value: String) -> BaseWidget<'a> {
  BaseWidget {
    wgt_type: WgtType::Text(TextType{text: value}),
    ch_type: ChType::None,
    top: 0.0,
    left: 0.0,
    width: 0.0,
    height: 0.0,
    parent: None,
  }
}

fn column<'a>(children: Vec<Box<&'a mut BaseWidget<'a>>>) -> BaseWidget<'a> {
  BaseWidget {
    wgt_type: WgtType::Column,
    ch_type: ChType::Children(children),
    top: 0.0,
    left: 0.0,
    width: 0.0,
    height: 0.0,
    parent: None,
  }
}

fn main() {
  // let mut app = App::new(Column::new(vec![Button::new()]));
  
  // app.body.get_base().set_pos(1.0,1.5);
  // app.body.get_base().get_pos();

  // app.body.get_base().get_children().as_mut().unwrap()[0].get_base().set_pos(2.0,3.5);
  // app.body.get_base().get_children().as_mut().unwrap()[0].get_base().get_pos();
  let mut t =text("Testing!".to_owned());
  let mut c = column(vec![Box::new(&mut t)]);

  c.build();
}
