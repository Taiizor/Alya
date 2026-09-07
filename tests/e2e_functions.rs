mod common;
use common::*;

#[test]
fn test_e2e_functions() {
    let code = r#"
function multiply(x, y)
    return x * y
end

say multiply(7, 6)
"#;
    if let Some(output) = run_alya_code(code) {
        assert_eq!(output, "42\n");
    }
}


#[test]
fn test_e2e_module_import() {
    let pid = std::process::id();
    let mod_filename = format!("temp_imported_helper_{}.alya", pid);
    let mod_content = r#"
function compute_bonus(salary)
    return salary * 2
end
"#;
    fs::write(&mod_filename, mod_content).expect("Failed to write temporary module file");

    let main_code = format!(
        r#"
import "{}"
let base = 1000
let total = compute_bonus(base)
say total
"#,
        mod_filename
    );

    let res = run_alya_code_full(&main_code);
    let _ = fs::remove_file(&mod_filename);

    if let Some((code, output)) = res {
        assert_eq!(code, 0);
        assert_eq!(output, "2000\n");
    }
}


#[test]
fn test_e2e_for_each_loop() {
    let code = r#"
let nums = [10, 20, 30]
for n in nums
    say n
end

for x in [1, 2, 3]
    say x * 2
end

let fruits = ["apple", "banana", "cherry"]
for f in fruits
    say "fruit: {f}"
end

for x in [1, 2, 3, 4, 5]
    if x == 2
        continue
    end
    if x == 4
        break
    end
    say x
end

let empty = []
for item in empty
    say "should not print"
end
"#;
    if let Some((code, output)) = run_alya_code_full(code) {
        assert_eq!(code, 0);
        assert_eq!(
            output,
            concat!(
                "10\n",
                "20\n",
                "30\n",
                "2\n",
                "4\n",
                "6\n",
                "fruit: apple\n",
                "fruit: banana\n",
                "fruit: cherry\n",
                "1\n",
                "3\n",
            )
        );
    }
}


#[test]
fn test_e2e_parameter_type_propagation() {
    let code = r#"
function append_tag(tags, val)
    tags.push(val)
end

function process(arr, dict, label)
    append_tag(arr, label)
    dict["tag"] = label
end

let items = ["init"]
let data = map()
process(items, data, "test_run")
say items[0]
say items[1]
say data["tag"]
"#;
    if let Some((code, output)) = run_alya_code_full(code) {
        assert_eq!(code, 0);
        assert_eq!(output, concat!("init\n", "test_run\n", "test_run\n",));
    }
}


