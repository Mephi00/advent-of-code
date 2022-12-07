#[derive(Clone)]
pub enum FSItem {
    File(File),
    Dir(Dir),
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct File {
    name: String,
    pub size: i32,
}

impl File {
    pub fn new(name: String, size: i32) -> File {
        File {
            name: name,
            size: size,
        }
    }
}

#[derive(Clone)]
pub struct Dir {
    pub name: String,
    items: Vec<FSItem>,
    pub size: i32,
}

impl Dir {
    pub fn new(name: String) -> Dir {
        Dir {
            name: name,
            items: Vec::new(),
            size: 0,
        }
    }

    pub fn add_item(&mut self, item: FSItem) {
        if let FSItem::File(file) = &item {
            self.size += file.size;
        } else if let FSItem::Dir(dir) = &item {
            self.size += dir.size;
        }

        self.items.push(item);
    }

    // pub fn recalc_size(&mut self) -> i32 {
    //     for mut item in &mut self.items.iter() {
    //         match item {
    //             FSItem::File(file) => {
    //                 self.size += file.size;
    //             }
    //             FSItem::Dir(dir) => {
    //                 self.size += dir.recalc_size();
    //             }
    //         }
    //     }

    //     self.size
    // }
}
