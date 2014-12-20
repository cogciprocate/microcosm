
//use std::num::FloatMath;
//use std::num::Float;

fn main() {

    let ent = Dude { name: "Fred", loc: 5i };
    let s_ent = ShittyDude { name: "Shitty Fred", loc: 13i };
    let f_er = ShittyFucker { dude: ShittyDude { name: "Shitty Fucker Fred", loc: 19i }, butthole: true, status: "fucked up" };
    
    
    ent.print();
    s_ent.print();
    f_er.print();
    
    println!("");
    
    print_ent(&ent);
    print_ent(&s_ent);
    print_ent(&f_er);
}


trait Printable {
    fn print(&self);
}


struct Dude {
    name: &'static str,
    loc: int,
}

impl Printable for Dude {
    fn print(&self) {
        println!("Dude - name: {}, loc: {}", self.name, self.loc);
    }
}


struct ShittyDude {
    name: &'static str,
    loc: int,
}

impl Printable for ShittyDude {
    fn print(&self) {
        println!("ShittyDudue - name: {}, loc: {}", self.name, self.loc);
    }
}


struct ShittyFucker {
    dude: ShittyDude,
    butthole: bool,
    status: &'static str,
}

impl Printable for ShittyFucker {
    fn print(&self) {
        println!("Fucker - name: {}, loc: {}, butthole: {}, loc: {}", self.dude.name, self.dude.loc, self.butthole, self.status);
    }
}


fn print_ent<T: Printable>(ent: &T) {
    ent.print()
}
