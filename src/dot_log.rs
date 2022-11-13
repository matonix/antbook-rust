// https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
use once_cell::sync::Lazy;
use petgraph::dot::Dot;
use petgraph::visit::{GraphProp, IntoEdgeReferences, IntoNodeReferences, NodeIndexable, NodeRef};
use std::fmt::Debug;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::sync::Mutex;

static DOT: Lazy<Mutex<Vec<(String, usize)>>> = Lazy::new(|| Mutex::new(vec![]));

pub fn log<G>(graph: G, level: usize)
where
  G: IntoEdgeReferences + IntoNodeReferences + NodeIndexable + GraphProp,
  G::EdgeWeight: Debug,
  G::NodeWeight: Debug,
{
  let v = vec!["0,0", "2,2", "4,0", "6,2", "8,0"];
  DOT.lock().unwrap().push((format!("{:?}", Dot::with_attr_getters(graph, &[], &|_, _| "".to_string(), &|g, n| format!("pos=\"{}!\"", v[g.to_index(n.id())]) )), level));
}

pub fn write_dot() {
  let template = read_to_string("template.html").expect("cannot read template file.");
  let mut out = File::create("dot.html").expect("cannot create log file.");
  writeln!(&mut out, "{template}").expect("cannot write template.");
  writeln!(&mut out, "var dots = [").expect("cannot write header.");
  let final_level = DOT
    .lock()
    .unwrap()
    .iter()
    .fold(0, |curr_level, (strs, level)| {
      writeln!(&mut out, "{}", brackets(curr_level, *level)).expect("cannot write brackets.");
      writeln!(&mut out, "[").expect("cannot write left bracket.");
      strs.lines().for_each(|l| writeln!(&mut out, "\t\t'{l}',").expect("cannot write line."));
      writeln!(&mut out, "],").expect("cannot write right bracket.");
      *level
    });
  writeln!(&mut out, "{}", brackets(final_level, 0)).expect("cannot write brackets.");
  writeln!(&mut out, "];\n</script>").expect("cannot write footer.");
  }

fn brackets(curr: usize, next: usize) -> String {
  if curr < next {
    (0..next-curr).map(|_| "[").collect::<String>()
  } else if curr > next {
    (0..curr-next).map(|_| "],").collect::<String>()
  } else {
    String::new()
  }
}