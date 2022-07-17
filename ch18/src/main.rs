struct Graph<T: Copy> {
    graph: Vec<Vertex<T>>,
}

struct Vertex<T: Copy> {
    value: T,
    visited: bool,
    adjacent: Vec<Vertex<T>>,
}

struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { queue: vec![] }
    }

    fn enqueue(&mut self, item: T) {
        self.queue.push(item);
    }

    fn dequeue(&mut self) {
        self.queue.remove(0);
    }

    fn read(&self) -> Option<&T> {
        self.queue.get(0)
    }
}

fn bfs_traversal<T>(mut vertex: &mut Vertex<T>)
where
    T: Copy,
{
    let mut queue: Queue<Vertex<T>> = Queue::new();

    loop {
        let read_value: Option<&Vertex<T>> = queue.read();

        if let Some(value) = read_value {
            vertex = &mut value;
        } else {
            break;
        }

        for adj_vertex in vertex.adjacent.iter_mut() {
            if !adj_vertex.visited {
                queue.enqueue(*adj_vertex); // fix this
                adj_vertex.visited = true;
            }
        }
    }
}

fn main() {}
