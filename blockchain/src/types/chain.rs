#[derive(Default)]
pub struct Node<T> {
    data: T,
    prev: Option<Box<Node<T>>>,
}

#[derive(Default)]
pub struct Chain<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}


impl<T: Default> Chain<T> {
    pub fn new() -> Chain<T> {
        Chain {
            // default values: len = 0
            //                 head = None
            ..Default::default()
        }
    }

    pub fn append(&mut self, item: T) {
        // забираем на некоторое время head
        let head = self.head.take();

        // добавляем новую ноду, которая ссылаеться на предыдущий
        // head, устанавливаем новый head на эту ноду,
        // добавляем размер
        let node = Box::new(Node {
            data: item,
            prev: head,
        });

        self.head = Some(node);
        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn head(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(head) => Some(&head.data),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chain_basic_test() {
        let mut chain = Chain::<u32>::new();

        chain.append(1);
        chain.append(2);
        chain.append(3);
        chain.append(4);

        assert_eq!(chain.head(), Some(&4))
    }
}