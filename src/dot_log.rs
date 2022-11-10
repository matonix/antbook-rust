// https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
use once_cell::sync::Lazy;
use petgraph::dot::Dot;
use petgraph::visit::{GraphProp, IntoEdgeReferences, IntoNodeReferences, NodeIndexable};
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
  DOT.lock().unwrap().push((format!("{:?}", Dot::new(graph)), level));
}

pub fn write_dot() {
  let template = read_to_string("template.html").expect("cannot read template file.");
  let mut out = File::create("dot.html").expect("cannot create log file.");
  writeln!(&mut out, "{template}").expect("cannot write template.");
  writeln!(&mut out, "var dots = [").expect("cannot write header.");
  DOT
    .lock()
    .unwrap()
    .iter()
    .fold(0, |curr_level, (strs, level)| {
      if curr_level < *level {
        writeln!(&mut out, "\t[").expect("cannot write left bracket.");
      } else if curr_level > * level {
        writeln!(&mut out, "\t],").expect("cannot write left bracket.");
      }
      writeln!(&mut out, "\t[").expect("cannot write left bracket.");
      strs.lines().for_each(|l| writeln!(&mut out, "\t\t'{l}',").expect("cannot write line."));
      writeln!(&mut out, "\t],").expect("cannot write right bracket.");
      *level
    });
  writeln!(&mut out, "];\n</script>").expect("cannot write footer.");
  }
