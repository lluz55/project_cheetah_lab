#![allow(unused)]
// trait Widget {}

// trait Component {
//   fn run(&mut self);
//   fn get_base(&mut self) -> &mut Base;
// }
// struct Base {
//   x: f32,
//   y: f32,
//   child: Option<Box::<dyn Component>>,
//   children: Option<Vec<Box::<dyn Component>>>,
// }

// impl Base {
//   pub fn default() -> Self {
//     Self {
//       x: 0.0,
//       y: 0.0,
//       child: None,
//       children: None,
//     }
//   }

//   pub fn with_child(child: Box::<dyn Component>) -> Base {
//     let mut base = Base::default();
//     base.child = Some(child);
//     base
//   }

//   pub fn with_children(children: Vec<Box::<dyn Component>>) -> Base {
//     let mut base = Base::default();
//     base.children = Some(children);
//     base
//   }
// }

// trait BaseComp {
//   fn get_pos(&self) -> &Base;
//   fn set_pos(&mut self, x: f32, y: f32);
//   fn get_child(&mut self) -> &mut Option<Box::<dyn Component>>;
//   fn get_children(&mut self) -> &mut Option<Vec<Box::<dyn Component>>>;
// }

// impl BaseComp for Base {
//   fn get_pos(&self) -> &Base {
//     println!("My position is: {}, {}", self.x, self.y);
//     &self
//   }

//   fn set_pos(&mut self, x: f32, y: f32) {
//     self.x = x;
//     self.y = y;
//   }

//   fn get_child(&mut self) -> &mut Option<Box::<dyn Component>> {
//     &mut self.child
//   }

//   fn get_children(&mut self) -> &mut Option<Vec<Box::<dyn Component>>> {
//     &mut self.children
//   }
// }

// struct Column {
//   base: Base,
// }

// impl Column {
//   pub fn new(children: Vec<Box::<dyn Component>>) -> Box::<Self> {
//     Box::new(Self{
//       base: Base::with_children(children),
//     })
//   }
// } 

// impl Component for Column {
//   fn run(&mut self) {
//     println!("column running!")
//   }

//   fn get_base(&mut self) -> &mut Base {
//       &mut self.base
//   }
// }
// struct Button {
//   pub base: Base,
// }

// impl Button {
//   pub fn new() -> Box::<dyn Component> {
//     Box::new(Button{
//       base: Base::default()
//     })
//   }

// }

// impl Component for Button {
//   fn run(&mut self) {
//     println!("i'm a button!")
//   }
//   fn get_base(&mut self) -> &mut Base {
//     &mut self.base
//   }

// }

// struct App {
//   pub body: Box::<dyn Component>,
//   pub base: Base,
// }

// impl App {
//   fn new(body: Box<dyn Component>) -> Self {
//     Self{
//       body,
//       base: Base::default()
//     }
//   }

//   pub fn run(&mut self) {
//     App::parse_child(&mut self.body)
//   }

//   fn parse_child(child: &mut Box::<dyn Component>) {

//   }

// }

pub trait Widget {
  fn get_wtype(&self) -> WType;
  fn get_ctype(&self) -> CType;

}

#[derive(Clone)]
pub struct TextType {
  text: String
}

pub enum WType {
  Text(TextType),
  Column,
}

pub enum CType {
  Child(Box::<BaseWidget>),
  Children(Vec<BaseWidget>),
  None,
}

use std::cell::{RefCell, RefMut};

pub struct BaseWidget {
  parent: Option<Box::<BaseWidget>>,
  w_type: WType,
  c_type: CType,
  top: f32,
  left: f32,
  width: f32,
  height: f32,
}

impl BaseWidget {

  pub fn new(ctype: CType, wtype: WType) -> BaseWidget {
    BaseWidget {
      w_type: wtype,
      c_type: ctype,
      top: 0.0,
      left: 0.0,
      width: 0.0,
      height: 0.0,
      parent: None,
    }
  }  

  pub fn run(&mut self) {
    let im_type = match self.w_type {
      WType::Column => "column",
      WType::Text(_) => "text",
    };
    println!("I'm a {} and my postion is x:{:.1} y:{:.1}", im_type, self.top, self.left);

    // TODO: Find out how to use RefCell
    // Checks Widgets type 
    match &self.w_type {
      // If widget is type Text then render text related stuff
      WType::Text(t) => { 
        println!("Rebuilding text");
        self.w_type = WType::Text(TextType{text: format!("New Value {}", t.text)});
        //println!("New text value {}", &t.text);
        
      },
      // If widget is type Column then calculate children size and set then they location
      WType::Column => {
        // TODO: Needs to find a way to cache childrens layout location
        // The widget type column is flexible and needs to first calculate all children with a
        // fixed size and divide the remaining height among the children with flexible size
        match &mut self.c_type {
          CType::Children(children) => {
            println!("Draw column");
            for child in children {
              child.set_position(1.0, 1.0);
              child.run();
              // TODO: Set child heigth and width
            }
          },
          _ => {}, // Column cant have type Child
        }
      }
    }
  }

  pub fn set_position(&mut self, top: f32, left: f32) {
    self.top = top;
  }
}


// TODO: Change functions to create widget to be a type
// TODO: that will allow more control over widget creation
fn text(value: String) -> BaseWidget {
  BaseWidget {
    w_type: WType::Text(TextType{text: value}),
    c_type: CType::None,
    top: 0.0,
    left: 0.0,
    width: 0.0,
    height: 0.0,
    parent: None,
  }
}

fn column<'a>(children: Vec<BaseWidget>) -> BaseWidget {
  BaseWidget {
    w_type: WType::Column,
    c_type: CType::Children(children),
    top: 0.0,
    left: 0.0,
    width: 0.0,
    height: 0.0,
    parent: None,
  }
}

// fn main() {

//   let mut app = App::new(Column::new(vec![Button::new()]));
  
//   app.body.get_base().set_pos(1.0,1.5);
//   app.body.get_base().get_pos();

//   app.body.get_base().get_children().as_mut().unwrap()[0].get_base().set_pos(2.0,3.5);
//   app.body.get_base().get_children().as_mut().unwrap()[0].get_base().get_pos();
//   let mut t1  = BaseWidget::new( CType::None, WType::Text(TextType{text: "Here".to_owned()}));
// }


fn main() {
  //let mut t =Text::build("Testing!".to_owned());
  // let mut c = column(vec![Box::new(&mut t1)]);
  let mut c = column(vec![
    text("Jere".to_owned())
  ]);

  c.run();
}
