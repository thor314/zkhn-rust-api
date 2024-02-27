use tracing::Level;
use tracing_subscriber::FmtSubscriber;

static INIT: std::sync::Once = std::sync::Once::new();
fn setup_test_tracing() {
  use tracing::Level;
  use tracing_subscriber::FmtSubscriber;

  INIT.call_once(|| {
    let subscriber =
      FmtSubscriber::builder().with_max_level(Level::INFO).with_test_writer().finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
  });
}
use rstest::{fixture, rstest};

// rstest provides features to take common context into tests, and set up small cases testing
#[derive(Clone, Debug, Eq, PartialEq)]
struct Wb {
  count: usize,
}

// context setup function to be implicitly called by `wb`
#[fixture]
fn count() -> usize { return 0usize; }

// context setup function to be implicitly called by `test_wb`
#[fixture]
fn wb(count: usize) -> Wb {
  setup_test_tracing();
  Wb { count }
}

#[rstest]
fn test_wb(wb: Wb) {
  tracing::info!("wb: {wb:?}");
  let Wb { count } = wb;
}
