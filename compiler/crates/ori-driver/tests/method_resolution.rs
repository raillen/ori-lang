use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use ori_driver::pipeline::{run_build, run_check, CheckOutput};

static NEXT_DIR_ID: AtomicU64 = AtomicU64::new(0);

struct TestDir {
    path: PathBuf,
}

impl TestDir {
    fn new(name: &str) -> Self {
        let id = NEXT_DIR_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "ori_driver_method_test_{}_{}_{}",
            std::process::id(),
            id,
            name,
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        Self { path }
    }

    fn path(&self, name: &str) -> PathBuf {
        self.path.join(name)
    }

    fn write(&self, name: &str, source: &str) {
        std::fs::write(self.path(name), source).unwrap();
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

fn diagnostic_codes(out: &CheckOutput) -> Vec<&'static str> {
    out.diagnostics.iter().map(|d| d.code).collect()
}

#[test]
fn build_lowers_inherent_method_call() {
    let dir = TestDir::new("inherent_method");
    dir.write(
        "main.orl",
        r#"module app.main

struct Player
    score: int

    add(self, bonus: int) -> int
        return self.score + bonus
    end
end

main()
    const player: Player = Player {score: 7}
    const total: int = player.add(5)
end
"#,
    );

    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);

    let build = run_build(&dir.path("main.orl")).unwrap();
    assert!(!build.has_errors, "{:?}", build.diagnostics);
    assert!(build
        .c_source
        .contains("ORI__app_dot_main_dot_Player_dot_add"));
    assert!(
        build
            .c_source
            .contains("ORI__app_dot_main_dot_Player_dot_add(player, INT64_C(5))"),
        "{}",
        build.c_source
    );
}

#[test]
fn check_reports_inherent_method_argument_type_mismatch() {
    let dir = TestDir::new("inherent_method_arg_type");
    dir.write(
        "main.orl",
        r#"module app.main

struct Player
    score: int

    add(self, bonus: int) -> int
        return self.score + bonus
    end
end

main()
    const player: Player = Player {score: 7}
    const total: int = player.add("bad")
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"type.arg_type_mismatch"));
}

#[test]
fn build_lowers_implement_method_call() {
    let dir = TestDir::new("implement_method");
    dir.write(
        "main.orl",
        r#"module app.main

struct Player
    score: int
end

trait Entity
    id(self) -> int
end

implement Entity for Player
    id(self) -> int
        return self.score
    end
end

main()
    const player: Player = Player {score: 42}
    const id: int = player.id()
end
"#,
    );

    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);

    let build = run_build(&dir.path("main.orl")).unwrap();
    assert!(!build.has_errors, "{:?}", build.diagnostics);
    assert!(build
        .c_source
        .contains("ORI__app_dot_main_dot_Player_dot_Entity_dot_id"));
    assert!(
        build
            .c_source
            .contains("ORI__app_dot_main_dot_Player_dot_Entity_dot_id(player)"),
        "{}",
        build.c_source
    );
}

#[test]
fn check_reports_ambiguous_trait_method_call() {
    let dir = TestDir::new("ambiguous_trait_method");
    dir.write(
        "main.orl",
        r#"module app.main

struct Thing
    name: string
end

trait Alpha
    output(self) -> string
end

trait Beta
    output(self) -> string
end

implement Alpha for Thing
    output(self) -> string
        return "alpha"
    end
end

implement Beta for Thing
    output(self) -> string
        return "beta"
    end
end

main()
    const thing: Thing = Thing {name: "x"}
    const text: string = thing.output()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors, "{:?}", out.diagnostics);
    assert!(diagnostic_codes(&out).contains(&"type.ambiguous_method"));
}

#[test]
fn build_lowers_qualified_trait_method_call() {
    let dir = TestDir::new("qualified_trait_method");
    dir.write(
        "main.orl",
        r#"module app.main

struct Thing
    name: string
end

trait Alpha
    output(self) -> string
end

trait Beta
    output(self) -> string
end

implement Alpha for Thing
    output(self) -> string
        return "alpha"
    end
end

implement Beta for Thing
    output(self) -> string
        return "beta"
    end
end

main()
    const thing: Thing = Thing {name: "x"}
    const alpha: string = Alpha.output(thing)
    const beta: string = Beta.output(thing)
end
"#,
    );

    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);

    let build = run_build(&dir.path("main.orl")).unwrap();
    assert!(!build.has_errors, "{:?}", build.diagnostics);
    assert!(build
        .c_source
        .contains("ORI__app_dot_main_dot_Thing_dot_Alpha_dot_output"));
    assert!(build
        .c_source
        .contains("ORI__app_dot_main_dot_Thing_dot_Beta_dot_output"));
}

#[test]
fn build_lowers_default_trait_method_call() {
    let dir = TestDir::new("default_trait_method");
    dir.write(
        "main.orl",
        r#"module app.main

struct Player
    score: int
end

trait Entity
    id(self) -> int
        return 7
    end
end

implement Entity for Player
end

main()
    const player: Player = Player {score: 42}
    const id: int = player.id()
end
"#,
    );

    let check = run_check(&dir.path("main.orl")).unwrap();
    assert!(!check.has_errors, "{:?}", check.diagnostics);

    let build = run_build(&dir.path("main.orl")).unwrap();
    assert!(!build.has_errors, "{:?}", build.diagnostics);
    assert!(build
        .c_source
        .contains("ORI__app_dot_main_dot_Entity_dot_id"));
}

#[test]
fn check_reports_missing_required_trait_method() {
    let dir = TestDir::new("missing_trait_method");
    dir.write(
        "main.orl",
        r#"module app.main

struct Player
end

trait Entity
    id(self) -> int
    tick(self) -> void
end

implement Entity for Player
    id(self) -> int
        return 1
    end
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"impl.missing_method"));
}

#[test]
fn check_reports_trait_method_signature_mismatch() {
    let dir = TestDir::new("trait_signature_mismatch");
    dir.write(
        "main.orl",
        r#"module app.main

struct Player
end

trait Cloneable
    merge(self, other: Self) -> Self
end

implement Cloneable for Player
    merge(self, other: int) -> Player
        return self
    end
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"impl.wrong_signature"));
}

#[test]
fn check_reports_trait_method_mut_mismatch() {
    let dir = TestDir::new("trait_mut_mismatch");
    dir.write(
        "main.orl",
        r#"module app.main

struct Bag
end

trait Stackable
    mut push(self) -> void
end

implement Stackable for Bag
    push(self) -> void
        return
    end
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"impl.mut_mismatch"));
}

#[test]
fn check_reports_implicit_self_mut_method_on_const_receiver() {
    let dir = TestDir::new("implicit_self_const_receiver");
    dir.write(
        "main.orl",
        r#"module app.main

struct Counter
    value: int

    mut increment()
        self.value = self.value + 1
    end
end

main()
    const counter: Counter = Counter {value: 1}
    counter.increment()
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"mut.const_method_call"));
}

#[test]
fn check_reports_duplicate_implement_pair() {
    let dir = TestDir::new("duplicate_implement");
    dir.write(
        "main.orl",
        r#"module app.main

struct Player
end

trait Entity
    id(self) -> int
end

implement Entity for Player
    id(self) -> int
        return 1
    end
end

implement Entity for Player
    id(self) -> int
        return 2
    end
end
"#,
    );

    let out = run_check(&dir.path("main.orl")).unwrap();
    assert!(out.has_errors);
    assert!(diagnostic_codes(&out).contains(&"bind.duplicate_implement"));
}
