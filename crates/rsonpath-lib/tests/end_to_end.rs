// bb34a4245e060e6f2147fa652c267816
use pretty_assertions::assert_eq;
use rsonpath::engine::{main::MainEngine, Compiler, Engine};
use rsonpath::input::*;
use rsonpath::query::JsonPathQuery;
use rsonpath::result::*;
use std::error::Error;
use std::fs;
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_compressed_with_query_look_for_the_atomic_value_forcing_a_tail_skip_of_the_large_object_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/skipping.toml running the query $.a.b (look for the atomic value, forcing a tail-skip of the large object) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/skipping.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_compressed_with_query_look_for_the_atomic_value_forcing_a_tail_skip_of_the_large_object_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/skipping.toml running the query $.a.b (look for the atomic value, forcing a tail-skip of the large object) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/skipping.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![453usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_compressed_with_query_look_for_the_atomic_value_forcing_a_tail_skip_of_the_large_object_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/skipping.toml running the query $.a.b (look for the atomic value, forcing a tail-skip of the large object) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/skipping.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_compressed_with_query_look_for_the_atomic_value_forcing_a_tail_skip_of_the_large_object_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/skipping.toml running the query $.a.b (look for the atomic value, forcing a tail-skip of the large object) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/skipping.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![453usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_compressed_with_query_look_for_the_atomic_value_forcing_a_tail_skip_of_the_large_object_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/skipping.toml running the query $.a.b (look for the atomic value, forcing a tail-skip of the large object) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/skipping.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_compressed_with_query_look_for_the_atomic_value_forcing_a_tail_skip_of_the_large_object_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/skipping.toml running the query $.a.b (look for the atomic value, forcing a tail-skip of the large object) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/skipping.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![453usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_compressed_with_query_look_for_the_atomic_value_with_descendant_forcing_memchr_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/skipping.toml running the query $..b (look for the atomic value with descendant, forcing memchr) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/skipping.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_compressed_with_query_look_for_the_atomic_value_with_descendant_forcing_memchr_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/skipping.toml running the query $..b (look for the atomic value with descendant, forcing memchr) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/skipping.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![453usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_compressed_with_query_look_for_the_atomic_value_with_descendant_forcing_memchr_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/skipping.toml running the query $..b (look for the atomic value with descendant, forcing memchr) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/skipping.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_compressed_with_query_look_for_the_atomic_value_with_descendant_forcing_memchr_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/skipping.toml running the query $..b (look for the atomic value with descendant, forcing memchr) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/skipping.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![453usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_compressed_with_query_look_for_the_atomic_value_with_descendant_forcing_memchr_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/skipping.toml running the query $..b (look for the atomic value with descendant, forcing memchr) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/skipping.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_compressed_with_query_look_for_the_atomic_value_with_descendant_forcing_memchr_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/skipping.toml running the query $..b (look for the atomic value with descendant, forcing memchr) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/skipping.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![453usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_with_query_look_for_the_atomic_value_forcing_a_tail_skip_of_the_large_object_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $.a.b (look for the atomic value, forcing a tail-skip of the large object) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/skipping.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_with_query_look_for_the_atomic_value_forcing_a_tail_skip_of_the_large_object_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $.a.b (look for the atomic value, forcing a tail-skip of the large object) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/skipping.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_with_query_look_for_the_atomic_value_forcing_a_tail_skip_of_the_large_object_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $.a.b (look for the atomic value, forcing a tail-skip of the large object) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/skipping.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_with_query_look_for_the_atomic_value_forcing_a_tail_skip_of_the_large_object_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $.a.b (look for the atomic value, forcing a tail-skip of the large object) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/skipping.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_with_query_look_for_the_atomic_value_forcing_a_tail_skip_of_the_large_object_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $.a.b (look for the atomic value, forcing a tail-skip of the large object) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/skipping.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_with_query_look_for_the_atomic_value_forcing_a_tail_skip_of_the_large_object_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $.a.b (look for the atomic value, forcing a tail-skip of the large object) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/skipping.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_with_query_look_for_the_atomic_value_with_descendant_forcing_memchr_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..b (look for the atomic value with descendant, forcing memchr) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/skipping.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_with_query_look_for_the_atomic_value_with_descendant_forcing_memchr_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..b (look for the atomic value with descendant, forcing memchr) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/skipping.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_with_query_look_for_the_atomic_value_with_descendant_forcing_memchr_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..b (look for the atomic value with descendant, forcing memchr) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/skipping.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_with_query_look_for_the_atomic_value_with_descendant_forcing_memchr_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..b (look for the atomic value with descendant, forcing memchr) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/skipping.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_with_query_look_for_the_atomic_value_with_descendant_forcing_memchr_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..b (look for the atomic value with descendant, forcing memchr) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/skipping.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_an_object_with_many_leaves_preceding_an_atomic_member_designed_to_stress_test_tail_skipping_with_query_look_for_the_atomic_value_with_descendant_forcing_memchr_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document skipping.toml running the query $..b (look for the atomic value with descendant, forcing memchr) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/skipping.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![810usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_deep_nesting_and_repeating_member_names_compressed_with_query_mix_descendant_and_child_names_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/child.toml running the query $..a..b.c..d (mix descendant and child names) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/child.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_deep_nesting_and_repeating_member_names_compressed_with_query_mix_descendant_and_child_names_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/child.toml running the query $..a..b.c..d (mix descendant and child names) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/child.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![100usize, 133usize, 153usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_deep_nesting_and_repeating_member_names_compressed_with_query_mix_descendant_and_child_names_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/child.toml running the query $..a..b.c..d (mix descendant and child names) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/child.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_deep_nesting_and_repeating_member_names_compressed_with_query_mix_descendant_and_child_names_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/child.toml running the query $..a..b.c..d (mix descendant and child names) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/child.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![100usize, 133usize, 153usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_deep_nesting_and_repeating_member_names_compressed_with_query_mix_descendant_and_child_names_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/child.toml running the query $..a..b.c..d (mix descendant and child names) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/child.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_deep_nesting_and_repeating_member_names_compressed_with_query_mix_descendant_and_child_names_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/child.toml running the query $..a..b.c..d (mix descendant and child names) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/child.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![100usize, 133usize, 153usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_deep_nesting_and_repeating_member_names_with_query_mix_descendant_and_child_names_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child.toml running the query $..a..b.c..d (mix descendant and child names) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/child.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_deep_nesting_and_repeating_member_names_with_query_mix_descendant_and_child_names_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child.toml running the query $..a..b.c..d (mix descendant and child names) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/child.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![986usize, 1299usize, 1547usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn artificial_json_with_deep_nesting_and_repeating_member_names_with_query_mix_descendant_and_child_names_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child.toml running the query $..a..b.c..d (mix descendant and child names) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/child.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_deep_nesting_and_repeating_member_names_with_query_mix_descendant_and_child_names_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child.toml running the query $..a..b.c..d (mix descendant and child names) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/child.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![986usize, 1299usize, 1547usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn artificial_json_with_deep_nesting_and_repeating_member_names_with_query_mix_descendant_and_child_names_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child.toml running the query $..a..b.c..d (mix descendant and child names) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/child.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_deep_nesting_and_repeating_member_names_with_query_mix_descendant_and_child_names_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child.toml running the query $..a..b.c..d (mix descendant and child names) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b.c..d")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/child.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![986usize, 1299usize, 1547usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_compressed_with_query_select_leaves_by_memchr_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/spaced_colon.toml running the query $..label (select leaves by memchr) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/spaced_colon.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_compressed_with_query_select_leaves_by_memchr_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/spaced_colon.toml running the query $..label (select leaves by memchr) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/spaced_colon.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![27usize, 40usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_compressed_with_query_select_leaves_by_memchr_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/spaced_colon.toml running the query $..label (select leaves by memchr) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/spaced_colon.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_compressed_with_query_select_leaves_by_memchr_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/spaced_colon.toml running the query $..label (select leaves by memchr) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/spaced_colon.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![27usize, 40usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_compressed_with_query_select_leaves_by_memchr_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/spaced_colon.toml running the query $..label (select leaves by memchr) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..label")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/spaced_colon.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_compressed_with_query_select_leaves_by_memchr_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/spaced_colon.toml running the query $..label (select leaves by memchr) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..label")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/spaced_colon.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![27usize, 40usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_compressed_with_query_select_leaves_tail_skipping_the_first_element_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/spaced_colon.toml running the query $.a..label (select leaves tail-skipping the first element) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/spaced_colon.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_compressed_with_query_select_leaves_tail_skipping_the_first_element_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/spaced_colon.toml running the query $.a..label (select leaves tail-skipping the first element) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/spaced_colon.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![27usize, 40usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_compressed_with_query_select_leaves_tail_skipping_the_first_element_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/spaced_colon.toml running the query $.a..label (select leaves tail-skipping the first element) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/spaced_colon.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_compressed_with_query_select_leaves_tail_skipping_the_first_element_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/spaced_colon.toml running the query $.a..label (select leaves tail-skipping the first element) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/spaced_colon.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![27usize, 40usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_compressed_with_query_select_leaves_tail_skipping_the_first_element_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/spaced_colon.toml running the query $.a..label (select leaves tail-skipping the first element) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..label")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/spaced_colon.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_compressed_with_query_select_leaves_tail_skipping_the_first_element_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/spaced_colon.toml running the query $.a..label (select leaves tail-skipping the first element) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..label")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/spaced_colon.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![27usize, 40usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_compressed_with_query_select_leaves_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/spaced_colon.toml running the query $..a..b..label (select leaves) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/spaced_colon.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_compressed_with_query_select_leaves_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/spaced_colon.toml running the query $..a..b..label (select leaves) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/spaced_colon.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![27usize, 40usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_compressed_with_query_select_leaves_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/spaced_colon.toml running the query $..a..b..label (select leaves) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/spaced_colon.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_compressed_with_query_select_leaves_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/spaced_colon.toml running the query $..a..b..label (select leaves) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/spaced_colon.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![27usize, 40usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_compressed_with_query_select_leaves_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/spaced_colon.toml running the query $..a..b..label (select leaves) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/spaced_colon.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_compressed_with_query_select_leaves_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/spaced_colon.toml running the query $..a..b..label (select leaves) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/spaced_colon.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![27usize, 40usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_with_query_select_leaves_by_memchr_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..label (select leaves by memchr) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/spaced_colon.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_with_query_select_leaves_by_memchr_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..label (select leaves by memchr) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/spaced_colon.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![150usize, 257usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_with_query_select_leaves_by_memchr_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..label (select leaves by memchr) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/spaced_colon.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_with_query_select_leaves_by_memchr_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..label (select leaves by memchr) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/spaced_colon.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![150usize, 257usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_with_query_select_leaves_by_memchr_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..label (select leaves by memchr) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..label")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/spaced_colon.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_with_query_select_leaves_by_memchr_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..label (select leaves by memchr) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..label")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/spaced_colon.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![150usize, 257usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_with_query_select_leaves_tail_skipping_the_first_element_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $.a..label (select leaves tail-skipping the first element) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/spaced_colon.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_with_query_select_leaves_tail_skipping_the_first_element_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $.a..label (select leaves tail-skipping the first element) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/spaced_colon.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![150usize, 257usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_with_query_select_leaves_tail_skipping_the_first_element_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $.a..label (select leaves tail-skipping the first element) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/spaced_colon.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_with_query_select_leaves_tail_skipping_the_first_element_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $.a..label (select leaves tail-skipping the first element) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/spaced_colon.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![150usize, 257usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_with_query_select_leaves_tail_skipping_the_first_element_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $.a..label (select leaves tail-skipping the first element) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..label")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/spaced_colon.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_with_query_select_leaves_tail_skipping_the_first_element_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $.a..label (select leaves tail-skipping the first element) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..label")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/spaced_colon.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![150usize, 257usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_with_query_select_leaves_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..a..b..label (select leaves) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/spaced_colon.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_with_query_select_leaves_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..a..b..label (select leaves) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/spaced_colon.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![150usize, 257usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_with_query_select_leaves_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..a..b..label (select leaves) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/spaced_colon.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_with_query_select_leaves_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..a..b..label (select leaves) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/spaced_colon.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![150usize, 257usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_with_query_select_leaves_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..a..b..label (select leaves) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/spaced_colon.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_excessive_whitespace_between_structural_colons_with_query_select_leaves_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document spaced_colon.toml running the query $..a..b..label (select leaves) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..label")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/spaced_colon.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![150usize, 257usize,], "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_many_equal_member_names_nested_in_each_other_to_stress_test_child_name_selectors_compressed_with_query_select_the_path_ababc_with_repeating_members_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc with repeating members) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/child_hell.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_many_equal_member_names_nested_in_each_other_to_stress_test_child_name_selectors_compressed_with_query_select_the_path_ababc_with_repeating_members_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc with repeating members) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/child_hell.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![40usize, 109usize, 190usize, 241usize, 264usize, 281usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn artificial_json_with_many_equal_member_names_nested_in_each_other_to_stress_test_child_name_selectors_compressed_with_query_select_the_path_ababc_with_repeating_members_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc with repeating members) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/child_hell.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_many_equal_member_names_nested_in_each_other_to_stress_test_child_name_selectors_compressed_with_query_select_the_path_ababc_with_repeating_members_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc with repeating members) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/child_hell.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![40usize, 109usize, 190usize, 241usize, 264usize, 281usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn artificial_json_with_many_equal_member_names_nested_in_each_other_to_stress_test_child_name_selectors_compressed_with_query_select_the_path_ababc_with_repeating_members_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc with repeating members) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/child_hell.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_many_equal_member_names_nested_in_each_other_to_stress_test_child_name_selectors_compressed_with_query_select_the_path_ababc_with_repeating_members_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc with repeating members) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/child_hell.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![40usize, 109usize, 190usize, 241usize, 264usize, 281usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn artificial_json_with_many_equal_member_names_nested_in_each_other_to_stress_test_child_name_selectors_with_query_select_the_path_ababc_with_repeating_members_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc with repeating members) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/child_hell.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_many_equal_member_names_nested_in_each_other_to_stress_test_child_name_selectors_with_query_select_the_path_ababc_with_repeating_members_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc with repeating members) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/child_hell.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![200usize, 758usize, 1229usize, 1905usize, 2042usize, 2209usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn artificial_json_with_many_equal_member_names_nested_in_each_other_to_stress_test_child_name_selectors_with_query_select_the_path_ababc_with_repeating_members_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc with repeating members) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/child_hell.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_many_equal_member_names_nested_in_each_other_to_stress_test_child_name_selectors_with_query_select_the_path_ababc_with_repeating_members_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc with repeating members) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/child_hell.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![200usize, 758usize, 1229usize, 1905usize, 2042usize, 2209usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn artificial_json_with_many_equal_member_names_nested_in_each_other_to_stress_test_child_name_selectors_with_query_select_the_path_ababc_with_repeating_members_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc with repeating members) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/child_hell.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn artificial_json_with_many_equal_member_names_nested_in_each_other_to_stress_test_child_name_selectors_with_query_select_the_path_ababc_with_repeating_members_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document child_hell.toml running the query $..x..a.b.a.b.c (select the path ababc with repeating members) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..x..a.b.a.b.c")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/child_hell.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![200usize, 758usize, 1229usize, 1905usize, 2042usize, 2209usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_compressed_with_query_select_all_nodes_in_bs_with_descendant_segments_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/deeply_nested_heterogenous_list.toml running the query $..a..*..b..* (select all nodes in bs with descendant segments) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let json_file =
        fs::File::open("../rsonpath-lib/tests/documents/json/compressed/deeply_nested_heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 12u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_compressed_with_query_select_all_nodes_in_bs_with_descendant_segments_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/deeply_nested_heterogenous_list.toml running the query $..a..*..b..* (select all nodes in bs with descendant segments) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let json_file =
        fs::File::open("../rsonpath-lib/tests/documents/json/compressed/deeply_nested_heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            33usize, 44usize, 46usize, 48usize, 51usize, 54usize, 55usize, 59usize, 65usize, 70usize, 76usize, 92usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_compressed_with_query_select_all_nodes_in_bs_with_descendant_segments_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/deeply_nested_heterogenous_list.toml running the query $..a..*..b..* (select all nodes in bs with descendant segments) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let json_file =
        fs::File::open("../rsonpath-lib/tests/documents/json/compressed/deeply_nested_heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 12u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_compressed_with_query_select_all_nodes_in_bs_with_descendant_segments_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/deeply_nested_heterogenous_list.toml running the query $..a..*..b..* (select all nodes in bs with descendant segments) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let json_file =
        fs::File::open("../rsonpath-lib/tests/documents/json/compressed/deeply_nested_heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            33usize, 44usize, 46usize, 48usize, 51usize, 54usize, 55usize, 59usize, 65usize, 70usize, 76usize, 92usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_compressed_with_query_select_all_nodes_in_bs_with_descendant_segments_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/deeply_nested_heterogenous_list.toml running the query $..a..*..b..* (select all nodes in bs with descendant segments) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let raw_json =
        fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/deeply_nested_heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 12u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_compressed_with_query_select_all_nodes_in_bs_with_descendant_segments_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/deeply_nested_heterogenous_list.toml running the query $..a..*..b..* (select all nodes in bs with descendant segments) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let raw_json =
        fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/deeply_nested_heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            33usize, 44usize, 46usize, 48usize, 51usize, 54usize, 55usize, 59usize, 65usize, 70usize, 76usize, 92usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_compressed_with_query_select_all_nodes_in_bs_with_mixed_segments_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/deeply_nested_heterogenous_list.toml running the query $..a.*..b.* (select all nodes in bs with mixed segments) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let json_file =
        fs::File::open("../rsonpath-lib/tests/documents/json/compressed/deeply_nested_heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_compressed_with_query_select_all_nodes_in_bs_with_mixed_segments_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/deeply_nested_heterogenous_list.toml running the query $..a.*..b.* (select all nodes in bs with mixed segments) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let json_file =
        fs::File::open("../rsonpath-lib/tests/documents/json/compressed/deeply_nested_heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![33usize, 44usize, 46usize, 48usize, 51usize, 54usize, 59usize, 65usize, 70usize, 76usize, 92usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_compressed_with_query_select_all_nodes_in_bs_with_mixed_segments_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/deeply_nested_heterogenous_list.toml running the query $..a.*..b.* (select all nodes in bs with mixed segments) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let json_file =
        fs::File::open("../rsonpath-lib/tests/documents/json/compressed/deeply_nested_heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_compressed_with_query_select_all_nodes_in_bs_with_mixed_segments_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/deeply_nested_heterogenous_list.toml running the query $..a.*..b.* (select all nodes in bs with mixed segments) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let json_file =
        fs::File::open("../rsonpath-lib/tests/documents/json/compressed/deeply_nested_heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![33usize, 44usize, 46usize, 48usize, 51usize, 54usize, 59usize, 65usize, 70usize, 76usize, 92usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_compressed_with_query_select_all_nodes_in_bs_with_mixed_segments_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/deeply_nested_heterogenous_list.toml running the query $..a.*..b.* (select all nodes in bs with mixed segments) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let raw_json =
        fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/deeply_nested_heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_compressed_with_query_select_all_nodes_in_bs_with_mixed_segments_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/deeply_nested_heterogenous_list.toml running the query $..a.*..b.* (select all nodes in bs with mixed segments) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let raw_json =
        fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/deeply_nested_heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![33usize, 44usize, 46usize, 48usize, 51usize, 54usize, 59usize, 65usize, 70usize, 76usize, 92usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_with_query_select_all_nodes_in_bs_with_descendant_segments_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deeply_nested_heterogenous_list.toml running the query $..a..*..b..* (select all nodes in bs with descendant segments) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/deeply_nested_heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 12u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_with_query_select_all_nodes_in_bs_with_descendant_segments_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deeply_nested_heterogenous_list.toml running the query $..a..*..b..* (select all nodes in bs with descendant segments) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/deeply_nested_heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            228usize, 401usize, 440usize, 479usize, 519usize, 559usize, 601usize, 679usize, 722usize, 764usize,
            807usize, 1012usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_with_query_select_all_nodes_in_bs_with_descendant_segments_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deeply_nested_heterogenous_list.toml running the query $..a..*..b..* (select all nodes in bs with descendant segments) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/deeply_nested_heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 12u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_with_query_select_all_nodes_in_bs_with_descendant_segments_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deeply_nested_heterogenous_list.toml running the query $..a..*..b..* (select all nodes in bs with descendant segments) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/deeply_nested_heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            228usize, 401usize, 440usize, 479usize, 519usize, 559usize, 601usize, 679usize, 722usize, 764usize,
            807usize, 1012usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_with_query_select_all_nodes_in_bs_with_descendant_segments_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deeply_nested_heterogenous_list.toml running the query $..a..*..b..* (select all nodes in bs with descendant segments) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/deeply_nested_heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 12u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_with_query_select_all_nodes_in_bs_with_descendant_segments_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deeply_nested_heterogenous_list.toml running the query $..a..*..b..* (select all nodes in bs with descendant segments) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*..b..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/deeply_nested_heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            228usize, 401usize, 440usize, 479usize, 519usize, 559usize, 601usize, 679usize, 722usize, 764usize,
            807usize, 1012usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_with_query_select_all_nodes_in_bs_with_mixed_segments_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deeply_nested_heterogenous_list.toml running the query $..a.*..b.* (select all nodes in bs with mixed segments) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/deeply_nested_heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_with_query_select_all_nodes_in_bs_with_mixed_segments_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deeply_nested_heterogenous_list.toml running the query $..a.*..b.* (select all nodes in bs with mixed segments) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/deeply_nested_heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            228usize, 401usize, 440usize, 479usize, 519usize, 559usize, 679usize, 722usize, 764usize, 807usize,
            1012usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_with_query_select_all_nodes_in_bs_with_mixed_segments_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deeply_nested_heterogenous_list.toml running the query $..a.*..b.* (select all nodes in bs with mixed segments) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/deeply_nested_heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_with_query_select_all_nodes_in_bs_with_mixed_segments_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deeply_nested_heterogenous_list.toml running the query $..a.*..b.* (select all nodes in bs with mixed segments) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/deeply_nested_heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            228usize, 401usize, 440usize, 479usize, 519usize, 559usize, 679usize, 722usize, 764usize, 807usize,
            1012usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_with_query_select_all_nodes_in_bs_with_mixed_segments_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deeply_nested_heterogenous_list.toml running the query $..a.*..b.* (select all nodes in bs with mixed segments) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/deeply_nested_heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_list_with_all_data_types_with_query_select_all_nodes_in_bs_with_mixed_segments_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document deeply_nested_heterogenous_list.toml running the query $..a.*..b.* (select all nodes in bs with mixed segments) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*..b.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/deeply_nested_heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            228usize, 401usize, 440usize, 479usize, 519usize, 559usize, 679usize, 722usize, 764usize, 807usize,
            1012usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_lists_compressed_with_query_select_all_nodes_in_the_top_level_list_and_below_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/matrioshka_list.toml running the query $..a..* (select all nodes in the top-level list and below) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/matrioshka_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_lists_compressed_with_query_select_all_nodes_in_the_top_level_list_and_below_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/matrioshka_list.toml running the query $..a..* (select all nodes in the top-level list and below) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/matrioshka_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 18usize, 20usize, 23usize, 26usize, 27usize, 28usize, 29usize, 35usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_lists_compressed_with_query_select_all_nodes_in_the_top_level_list_and_below_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/matrioshka_list.toml running the query $..a..* (select all nodes in the top-level list and below) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/matrioshka_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_lists_compressed_with_query_select_all_nodes_in_the_top_level_list_and_below_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/matrioshka_list.toml running the query $..a..* (select all nodes in the top-level list and below) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/matrioshka_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 18usize, 20usize, 23usize, 26usize, 27usize, 28usize, 29usize, 35usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_lists_compressed_with_query_select_all_nodes_in_the_top_level_list_and_below_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/matrioshka_list.toml running the query $..a..* (select all nodes in the top-level list and below) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/matrioshka_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_lists_compressed_with_query_select_all_nodes_in_the_top_level_list_and_below_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/matrioshka_list.toml running the query $..a..* (select all nodes in the top-level list and below) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/matrioshka_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 18usize, 20usize, 23usize, 26usize, 27usize, 28usize, 29usize, 35usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_lists_compressed_with_query_select_all_nodes_in_the_top_level_list_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/matrioshka_list.toml running the query $..a.* (select all nodes in the top-level list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/matrioshka_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_lists_compressed_with_query_select_all_nodes_in_the_top_level_list_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/matrioshka_list.toml running the query $..a.* (select all nodes in the top-level list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/matrioshka_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 18usize, 20usize, 23usize, 26usize, 35usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_lists_compressed_with_query_select_all_nodes_in_the_top_level_list_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/matrioshka_list.toml running the query $..a.* (select all nodes in the top-level list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/matrioshka_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_lists_compressed_with_query_select_all_nodes_in_the_top_level_list_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/matrioshka_list.toml running the query $..a.* (select all nodes in the top-level list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/matrioshka_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 18usize, 20usize, 23usize, 26usize, 35usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_lists_compressed_with_query_select_all_nodes_in_the_top_level_list_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/matrioshka_list.toml running the query $..a.* (select all nodes in the top-level list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/matrioshka_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_lists_compressed_with_query_select_all_nodes_in_the_top_level_list_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/matrioshka_list.toml running the query $..a.* (select all nodes in the top-level list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/matrioshka_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 18usize, 20usize, 23usize, 26usize, 35usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_lists_with_query_select_all_nodes_in_the_top_level_list_and_below_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document matrioshka_list.toml running the query $..a..* (select all nodes in the top-level list and below) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/matrioshka_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_lists_with_query_select_all_nodes_in_the_top_level_list_and_below_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document matrioshka_list.toml running the query $..a..* (select all nodes in the top-level list and below) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/matrioshka_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![63usize, 82usize, 101usize, 121usize, 141usize, 163usize, 189usize, 219usize, 305usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_lists_with_query_select_all_nodes_in_the_top_level_list_and_below_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document matrioshka_list.toml running the query $..a..* (select all nodes in the top-level list and below) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/matrioshka_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_lists_with_query_select_all_nodes_in_the_top_level_list_and_below_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document matrioshka_list.toml running the query $..a..* (select all nodes in the top-level list and below) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/matrioshka_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![63usize, 82usize, 101usize, 121usize, 141usize, 163usize, 189usize, 219usize, 305usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_lists_with_query_select_all_nodes_in_the_top_level_list_and_below_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document matrioshka_list.toml running the query $..a..* (select all nodes in the top-level list and below) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/matrioshka_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_lists_with_query_select_all_nodes_in_the_top_level_list_and_below_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document matrioshka_list.toml running the query $..a..* (select all nodes in the top-level list and below) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/matrioshka_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![63usize, 82usize, 101usize, 121usize, 141usize, 163usize, 189usize, 219usize, 305usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_lists_with_query_select_all_nodes_in_the_top_level_list_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document matrioshka_list.toml running the query $..a.* (select all nodes in the top-level list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/matrioshka_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_lists_with_query_select_all_nodes_in_the_top_level_list_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document matrioshka_list.toml running the query $..a.* (select all nodes in the top-level list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/matrioshka_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![63usize, 82usize, 101usize, 121usize, 141usize, 305usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_lists_with_query_select_all_nodes_in_the_top_level_list_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document matrioshka_list.toml running the query $..a.* (select all nodes in the top-level list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/matrioshka_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_lists_with_query_select_all_nodes_in_the_top_level_list_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document matrioshka_list.toml running the query $..a.* (select all nodes in the top-level list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/matrioshka_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![63usize, 82usize, 101usize, 121usize, 141usize, 305usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_lists_with_query_select_all_nodes_in_the_top_level_list_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document matrioshka_list.toml running the query $..a.* (select all nodes in the top-level list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/matrioshka_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_lists_with_query_select_all_nodes_in_the_top_level_list_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document matrioshka_list.toml running the query $..a.* (select all nodes in the top-level list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/matrioshka_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![63usize, 82usize, 101usize, 121usize, 141usize, 305usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_object_with_path_annotations_compressed_with_query_descendant_a_star_star_then_descendant_b_star_star_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/long_path.toml running the query $..a.*.*..b.*.* (descendant a star star, then descendant b star star) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/long_path.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_object_with_path_annotations_compressed_with_query_descendant_a_star_star_then_descendant_b_star_star_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/long_path.toml running the query $..a.*.*..b.*.* (descendant a star star, then descendant b star star) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/long_path.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![253usize, 280usize, 290usize, 298usize, 328usize, 338usize, 570usize, 614usize, 625usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_object_with_path_annotations_compressed_with_query_descendant_a_star_star_then_descendant_b_star_star_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/long_path.toml running the query $..a.*.*..b.*.* (descendant a star star, then descendant b star star) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/long_path.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_object_with_path_annotations_compressed_with_query_descendant_a_star_star_then_descendant_b_star_star_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/long_path.toml running the query $..a.*.*..b.*.* (descendant a star star, then descendant b star star) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/long_path.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![253usize, 280usize, 290usize, 298usize, 328usize, 338usize, 570usize, 614usize, 625usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_object_with_path_annotations_compressed_with_query_descendant_a_star_star_then_descendant_b_star_star_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/long_path.toml running the query $..a.*.*..b.*.* (descendant a star star, then descendant b star star) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/long_path.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_object_with_path_annotations_compressed_with_query_descendant_a_star_star_then_descendant_b_star_star_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/long_path.toml running the query $..a.*.*..b.*.* (descendant a star star, then descendant b star star) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/long_path.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![253usize, 280usize, 290usize, 298usize, 328usize, 338usize, 570usize, 614usize, 625usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_object_with_path_annotations_with_query_descendant_a_star_star_then_descendant_b_star_star_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document long_path.toml running the query $..a.*.*..b.*.* (descendant a star star, then descendant b star star) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/long_path.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_object_with_path_annotations_with_query_descendant_a_star_star_then_descendant_b_star_star_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document long_path.toml running the query $..a.*.*..b.*.* (descendant a star star, then descendant b star star) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/long_path.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![654usize, 711usize, 751usize, 793usize, 857usize, 901usize, 1715usize, 1813usize, 1878usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_object_with_path_annotations_with_query_descendant_a_star_star_then_descendant_b_star_star_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document long_path.toml running the query $..a.*.*..b.*.* (descendant a star star, then descendant b star star) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/long_path.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_object_with_path_annotations_with_query_descendant_a_star_star_then_descendant_b_star_star_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document long_path.toml running the query $..a.*.*..b.*.* (descendant a star star, then descendant b star star) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/long_path.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![654usize, 711usize, 751usize, 793usize, 857usize, 901usize, 1715usize, 1813usize, 1878usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn deeply_nested_object_with_path_annotations_with_query_descendant_a_star_star_then_descendant_b_star_star_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document long_path.toml running the query $..a.*.*..b.*.* (descendant a star star, then descendant b star star) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/long_path.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9u64, "result != expected");
    Ok(())
}
#[test]
fn deeply_nested_object_with_path_annotations_with_query_descendant_a_star_star_then_descendant_b_star_star_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document long_path.toml running the query $..a.*.*..b.*.* (descendant a star star, then descendant b star star) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*.*..b.*.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/long_path.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![654usize, 711usize, 751usize, 793usize, 857usize, 901usize, 1715usize, 1813usize, 1878usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_compressed_with_query_descendant_search_for_a_and_then_all_subdocuments_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/dense_atomic_leaves.toml running the query $..a..* (descendant search for 'a' and then all subdocuments) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/dense_atomic_leaves.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_compressed_with_query_descendant_search_for_a_and_then_all_subdocuments_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/dense_atomic_leaves.toml running the query $..a..* (descendant search for 'a' and then all subdocuments) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/dense_atomic_leaves.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![20usize, 27usize, 34usize, 41usize, 46usize, 54usize, 59usize, 66usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_compressed_with_query_descendant_search_for_a_and_then_all_subdocuments_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/dense_atomic_leaves.toml running the query $..a..* (descendant search for 'a' and then all subdocuments) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/dense_atomic_leaves.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_compressed_with_query_descendant_search_for_a_and_then_all_subdocuments_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/dense_atomic_leaves.toml running the query $..a..* (descendant search for 'a' and then all subdocuments) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/dense_atomic_leaves.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![20usize, 27usize, 34usize, 41usize, 46usize, 54usize, 59usize, 66usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_compressed_with_query_descendant_search_for_a_and_then_all_subdocuments_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/dense_atomic_leaves.toml running the query $..a..* (descendant search for 'a' and then all subdocuments) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/dense_atomic_leaves.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_compressed_with_query_descendant_search_for_a_and_then_all_subdocuments_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/dense_atomic_leaves.toml running the query $..a..* (descendant search for 'a' and then all subdocuments) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/dense_atomic_leaves.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![20usize, 27usize, 34usize, 41usize, 46usize, 54usize, 59usize, 66usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_compressed_with_query_descendant_search_for_a_and_then_take_all_children_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/dense_atomic_leaves.toml running the query $..a.* (descendant search for 'a' and then take all children) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/dense_atomic_leaves.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_compressed_with_query_descendant_search_for_a_and_then_take_all_children_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/dense_atomic_leaves.toml running the query $..a.* (descendant search for 'a' and then take all children) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/dense_atomic_leaves.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![20usize, 27usize, 34usize, 41usize, 54usize, 59usize, 66usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_compressed_with_query_descendant_search_for_a_and_then_take_all_children_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/dense_atomic_leaves.toml running the query $..a.* (descendant search for 'a' and then take all children) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/dense_atomic_leaves.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_compressed_with_query_descendant_search_for_a_and_then_take_all_children_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/dense_atomic_leaves.toml running the query $..a.* (descendant search for 'a' and then take all children) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/dense_atomic_leaves.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![20usize, 27usize, 34usize, 41usize, 54usize, 59usize, 66usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_compressed_with_query_descendant_search_for_a_and_then_take_all_children_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/dense_atomic_leaves.toml running the query $..a.* (descendant search for 'a' and then take all children) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/dense_atomic_leaves.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_compressed_with_query_descendant_search_for_a_and_then_take_all_children_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/dense_atomic_leaves.toml running the query $..a.* (descendant search for 'a' and then take all children) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/dense_atomic_leaves.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![20usize, 27usize, 34usize, 41usize, 54usize, 59usize, 66usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_with_query_descendant_search_for_a_and_then_all_subdocuments_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_atomic_leaves.toml running the query $..a..* (descendant search for 'a' and then all subdocuments) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/dense_atomic_leaves.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_with_query_descendant_search_for_a_and_then_all_subdocuments_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_atomic_leaves.toml running the query $..a..* (descendant search for 'a' and then all subdocuments) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/dense_atomic_leaves.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![68usize, 93usize, 118usize, 143usize, 170usize, 213usize, 240usize, 269usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_with_query_descendant_search_for_a_and_then_all_subdocuments_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_atomic_leaves.toml running the query $..a..* (descendant search for 'a' and then all subdocuments) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/dense_atomic_leaves.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_with_query_descendant_search_for_a_and_then_all_subdocuments_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_atomic_leaves.toml running the query $..a..* (descendant search for 'a' and then all subdocuments) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/dense_atomic_leaves.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![68usize, 93usize, 118usize, 143usize, 170usize, 213usize, 240usize, 269usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_with_query_descendant_search_for_a_and_then_all_subdocuments_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_atomic_leaves.toml running the query $..a..* (descendant search for 'a' and then all subdocuments) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/dense_atomic_leaves.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_with_query_descendant_search_for_a_and_then_all_subdocuments_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_atomic_leaves.toml running the query $..a..* (descendant search for 'a' and then all subdocuments) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/dense_atomic_leaves.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![68usize, 93usize, 118usize, 143usize, 170usize, 213usize, 240usize, 269usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_with_query_descendant_search_for_a_and_then_take_all_children_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_atomic_leaves.toml running the query $..a.* (descendant search for 'a' and then take all children) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/dense_atomic_leaves.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_with_query_descendant_search_for_a_and_then_take_all_children_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_atomic_leaves.toml running the query $..a.* (descendant search for 'a' and then take all children) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/dense_atomic_leaves.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![68usize, 93usize, 118usize, 143usize, 213usize, 240usize, 269usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_with_query_descendant_search_for_a_and_then_take_all_children_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_atomic_leaves.toml running the query $..a.* (descendant search for 'a' and then take all children) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/dense_atomic_leaves.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_with_query_descendant_search_for_a_and_then_take_all_children_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_atomic_leaves.toml running the query $..a.* (descendant search for 'a' and then take all children) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/dense_atomic_leaves.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![68usize, 93usize, 118usize, 143usize, 213usize, 240usize, 269usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_with_query_descendant_search_for_a_and_then_take_all_children_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_atomic_leaves.toml running the query $..a.* (descendant search for 'a' and then take all children) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/dense_atomic_leaves.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn dense_integer_atomic_leaves_without_lists_with_query_descendant_search_for_a_and_then_take_all_children_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document dense_atomic_leaves.toml running the query $..a.* (descendant search for 'a' and then take all children) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/dense_atomic_leaves.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![68usize, 93usize, 118usize, 143usize, 213usize, 240usize, 269usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_descendant_there_are_none_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_descendant_there_are_none_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_descendant_there_are_none_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_descendant_there_are_none_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_item_there_are_none_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $[*] (select any item (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_item_there_are_none_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $[*] (select any item (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_item_there_are_none_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $[*] (select any item (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_item_there_are_none_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $[*] (select any item (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_item_there_are_none_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $[*] (select any item (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_any_item_there_are_none_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $[*] (select any item (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_first_item_which_does_not_exist_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_first_item_which_does_not_exist_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_first_item_which_does_not_exist_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_first_item_which_does_not_exist_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_first_item_which_does_not_exist_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_first_item_which_does_not_exist_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_empty_query_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_empty_query_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_empty_query_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_empty_query_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_empty_query_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_empty_query_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $ (select the root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $ (select the root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $ (select the root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $ (select the root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $ (select the root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_compressed_with_query_select_the_root_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_array.toml running the query $ (select the root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_descendant_there_are_none_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_descendant_there_are_none_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_descendant_there_are_none_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_descendant_there_are_none_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_item_there_are_none_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_item_there_are_none_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_item_there_are_none_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_item_there_are_none_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_item_there_are_none_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_any_item_there_are_none_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[*] (select any item (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[*]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_first_item_which_does_not_exist_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_first_item_which_does_not_exist_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_first_item_which_does_not_exist_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_first_item_which_does_not_exist_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_first_item_which_does_not_exist_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_first_item_which_does_not_exist_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $[0] (select the first item (which does not exist)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_empty_query_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_empty_query_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_empty_query_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_empty_query_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_empty_query_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_empty_query_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_array_root_with_query_select_the_root_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_array.toml running the query $ (select the root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty_array.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_empty_query_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_empty_query_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_empty_query_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_empty_query_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_empty_query_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_empty_query_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty.toml running the query $ (select the root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty.toml running the query $ (select the root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty.toml running the query $ (select the root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty.toml running the query $ (select the root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty.toml running the query $ (select the root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_compressed_with_query_select_the_root_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty.toml running the query $ (select the root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_empty_query_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_empty_query_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_empty_query_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_empty_query_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_empty_query_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_empty_query_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_document_with_query_select_the_root_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty.toml running the query $ (select the root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_child_there_are_none_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $.* (select any child (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_child_there_are_none_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $.* (select any child (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_child_there_are_none_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $.* (select any child (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_child_there_are_none_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $.* (select any child (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_child_there_are_none_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $.* (select any child (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_child_there_are_none_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $.* (select any child (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_descendant_there_are_none_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_descendant_there_are_none_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_descendant_there_are_none_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_descendant_there_are_none_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_child_named_a_which_does_not_exist_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_child_named_a_which_does_not_exist_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_child_named_a_which_does_not_exist_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_child_named_a_which_does_not_exist_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_child_named_a_which_does_not_exist_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_child_named_a_which_does_not_exist_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_empty_query_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_empty_query_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_empty_query_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_empty_query_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_empty_query_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_empty_query_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $ (select the root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $ (select the root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $ (select the root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $ (select the root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $ (select the root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_compressed_with_query_select_the_root_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/empty_object.toml running the query $ (select the root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_child_there_are_none_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_child_there_are_none_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_child_there_are_none_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_child_there_are_none_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_child_there_are_none_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_child_there_are_none_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.* (select any child (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_descendant_there_are_none_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_descendant_there_are_none_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_descendant_there_are_none_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_descendant_there_are_none_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_any_descendant_there_are_none_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $..* (select any descendant (there are none)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_child_named_a_which_does_not_exist_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_child_named_a_which_does_not_exist_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_child_named_a_which_does_not_exist_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_child_named_a_which_does_not_exist_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_child_named_a_which_does_not_exist_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_child_named_a_which_does_not_exist_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $.a (select the child named 'a' (which does not exist)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_empty_query_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_empty_query_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_empty_query_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_empty_query_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_empty_query_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_empty_query_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query  (select the root (empty query)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn empty_object_root_with_query_select_the_root_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document empty_object.toml running the query $ (select the root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/empty_object.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_entities_then_url_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_entities_then_url_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![151usize, 198usize, 341usize, 388usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_entities_then_url_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_entities_then_url_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![151usize, 198usize, 341usize, 388usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_entities_then_url_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_entities_then_url_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![151usize, 198usize, 341usize, 388usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize, 341usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize, 341usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize, 341usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_urls_arrays_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_urls_arrays_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize, 341usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_urls_arrays_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_urls_arrays_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize, 341usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_urls_arrays_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_for_url_limited_to_urls_arrays_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize, 341usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_search_for_url_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..url (descendant search for url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_search_for_url_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..url (descendant search for url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 90usize, 151usize, 198usize, 267usize, 341usize, 388usize, 426usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_search_for_url_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..url (descendant search for url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_search_for_url_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..url (descendant search for url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 90usize, 151usize, 198usize, 267usize, 341usize, 388usize, 426usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_search_for_url_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..url (descendant search for url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_descendant_search_for_url_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $..url (descendant search for url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 90usize, 151usize, 198usize, 267usize, 341usize, 388usize, 426usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_direct_path_to_the_top_level_url_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_direct_path_to_the_top_level_url_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![426usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_direct_path_to_the_top_level_url_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_direct_path_to_the_top_level_url_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![426usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_direct_path_to_the_top_level_url_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_compressed_with_query_direct_path_to_the_top_level_url_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![426usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_entities_then_url_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_entities_then_url_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![323usize, 473usize, 883usize, 1013usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_entities_then_url_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_entities_then_url_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![323usize, 473usize, 883usize, 1013usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_entities_then_url_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_entities_then_url_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..url (descendant entities then url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![323usize, 473usize, 883usize, 1013usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![323usize, 883usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![323usize, 883usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_direct_urls_arrays_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities.urls..url (descendant for url limited to direct urls arrays) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities.urls..url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![323usize, 883usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_urls_arrays_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_urls_arrays_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![323usize, 883usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_urls_arrays_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_urls_arrays_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![323usize, 883usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_urls_arrays_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_for_url_limited_to_urls_arrays_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..entities..urls..url (descendant for url limited to urls arrays) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..entities..urls..url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![323usize, 883usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_search_for_url_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_search_for_url_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![45usize, 170usize, 323usize, 473usize, 672usize, 883usize, 1013usize, 1100usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_search_for_url_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_search_for_url_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![45usize, 170usize, 323usize, 473usize, 672usize, 883usize, 1013usize, 1100usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_search_for_url_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 8u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_descendant_search_for_url_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $..url (descendant search for url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![45usize, 170usize, 323usize, 473usize, 672usize, 883usize, 1013usize, 1100usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_direct_path_to_the_top_level_url_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_direct_path_to_the_top_level_url_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![1100usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_direct_path_to_the_top_level_url_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_direct_path_to_the_top_level_url_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![1100usize,], "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_direct_path_to_the_top_level_url_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn extract_from_twitter_json_containing_urls_with_multiple_escaped_slashes_with_query_direct_path_to_the_top_level_url_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter_urls.toml running the query $[0].url (direct path to the top-level url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/twitter_urls.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![1100usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_a_leading_quote_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_a_leading_quote_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![12usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_a_leading_quote_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_a_leading_quote_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![12usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_a_leading_quote_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_a_leading_quote_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![12usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![19usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![19usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_compressed_with_query_descendant_search_for_b_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![19usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_a_leading_quote_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_a_leading_quote_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![28usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_a_leading_quote_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_a_leading_quote_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![28usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_a_leading_quote_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_a_leading_quote_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..['\"b'] (descendant search for 'b' with a leading quote) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..['\"b']")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![28usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/memchr_trap.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![45usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/memchr_trap.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![45usize,], "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn label_b_and_b_with_escaped_quote_to_trick_naive_string_comparison_with_query_descendant_search_for_b_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document memchr_trap.toml running the query $..b (descendant search for 'b') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/memchr_trap.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![45usize,], "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_claims_references_hash_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..claims..references..hash (descendant search for claims references hash) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..claims..references..hash")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 37736u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_claims_references_hash_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..claims..references..hash (descendant search for claims references hash) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..claims..references..hash")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 37736u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_claims_references_hash_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..claims..references..hash (descendant search for claims references hash) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..claims..references..hash")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 37736u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_datavalue_value_id_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..datavalue..value..id (descendant search for datavalue value id) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..datavalue..value..id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25093u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_datavalue_value_id_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..datavalue..value..id (descendant search for datavalue value id) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..datavalue..value..id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25093u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_datavalue_value_id_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..datavalue..value..id (descendant search for datavalue value id) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..datavalue..value..id")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25093u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_en_then_directly_for_value_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..en.value (descendant search for en, then directly for value) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en.value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1753u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_en_then_directly_for_value_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..en.value (descendant search for en, then directly for value) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en.value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1753u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_en_then_directly_for_value_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..en.value (descendant search for en, then directly for value) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en.value")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1753u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_en_value_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..en..value (descendant search for en value) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2360u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_en_value_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..en..value (descendant search for en value) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2360u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_en_value_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..en..value (descendant search for en value) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en..value")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2360u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_mainsnak_datavalue_value_id_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..mainsnak..datavalue..value..id (descendant search for mainsnak datavalue value id) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..mainsnak..datavalue..value..id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 12958u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_mainsnak_datavalue_value_id_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..mainsnak..datavalue..value..id (descendant search for mainsnak datavalue value id) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..mainsnak..datavalue..value..id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 12958u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_mainsnak_datavalue_value_id_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..mainsnak..datavalue..value..id (descendant search for mainsnak datavalue value id) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..mainsnak..datavalue..value..id")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 12958u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_mainsnak_datavalue_value_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..mainsnak..datavalue..value (descendant search for mainsnak datavalue value) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..mainsnak..datavalue..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 26115u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_mainsnak_datavalue_value_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..mainsnak..datavalue..value (descendant search for mainsnak datavalue value) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..mainsnak..datavalue..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 26115u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_mainsnak_datavalue_value_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..mainsnak..datavalue..value (descendant search for mainsnak datavalue value) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..mainsnak..datavalue..value")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 26115u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_references_snaks_datavalue_value_id_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..references..snaks..datavalue..value..id (descendant search for references snaks datavalue value id) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..references..snaks..datavalue..value..id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11113u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_references_snaks_datavalue_value_id_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..references..snaks..datavalue..value..id (descendant search for references snaks datavalue value id) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..references..snaks..datavalue..value..id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11113u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_references_snaks_datavalue_value_id_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..references..snaks..datavalue..value..id (descendant search for references snaks datavalue value id) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..references..snaks..datavalue..value..id")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11113u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_references_snaks_datavalue_value_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..references..snaks..datavalue..value (descendant search for references snaks datavalue value) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..references..snaks..datavalue..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25118u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_references_snaks_datavalue_value_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..references..snaks..datavalue..value (descendant search for references snaks datavalue value) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..references..snaks..datavalue..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25118u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_references_snaks_datavalue_value_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..references..snaks..datavalue..value (descendant search for references snaks datavalue value) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..references..snaks..datavalue..value")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25118u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_references_snaks_datavalue_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..references..snaks..datavalue (descendant search for references snaks datavalue) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..references..snaks..datavalue")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25118u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_references_snaks_datavalue_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..references..snaks..datavalue (descendant search for references snaks datavalue) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..references..snaks..datavalue")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25118u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_references_snaks_datavalue_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..references..snaks..datavalue (descendant search for references snaks datavalue) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..references..snaks..datavalue")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25118u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_snaks_datavalue_value_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..snaks..datavalue..value (descendant search for snaks datavalue value) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..snaks..datavalue..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25118u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_snaks_datavalue_value_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..snaks..datavalue..value (descendant search for snaks datavalue value) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..snaks..datavalue..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25118u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_snaks_datavalue_value_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..snaks..datavalue..value (descendant search for snaks datavalue value) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..snaks..datavalue..value")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25118u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_snaks_then_any_descendant_and_its_id_child_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..snaks..*.id (descendant search for snaks, then any descendant and its id child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..snaks..*.id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11113u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_snaks_then_any_descendant_and_its_id_child_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..snaks..*.id (descendant search for snaks, then any descendant and its id child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..snaks..*.id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11113u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_descendant_search_for_snaks_then_any_descendant_and_its_id_child_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..snaks..*.id (descendant search for snaks, then any descendant and its id child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..snaks..*.id")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11113u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_select_all_nodes_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..* (select all nodes) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 970442u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_select_all_nodes_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..* (select all nodes) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 970442u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_compressed_with_query_select_all_nodes_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_person.toml running the query $..* (select all nodes) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 970442u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_claims_references_hash_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..claims..references..hash (descendant search for claims references hash) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..claims..references..hash")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 37736u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_claims_references_hash_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..claims..references..hash (descendant search for claims references hash) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..claims..references..hash")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 37736u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_claims_references_hash_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..claims..references..hash (descendant search for claims references hash) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..claims..references..hash")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 37736u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_datavalue_value_id_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..datavalue..value..id (descendant search for datavalue value id) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..datavalue..value..id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25093u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_datavalue_value_id_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..datavalue..value..id (descendant search for datavalue value id) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..datavalue..value..id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25093u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_datavalue_value_id_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..datavalue..value..id (descendant search for datavalue value id) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..datavalue..value..id")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25093u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_en_then_directly_for_value_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..en.value (descendant search for en, then directly for value) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en.value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1753u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_en_then_directly_for_value_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..en.value (descendant search for en, then directly for value) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en.value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1753u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_en_then_directly_for_value_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..en.value (descendant search for en, then directly for value) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en.value")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1753u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_en_value_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..en..value (descendant search for en value) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2360u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_en_value_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..en..value (descendant search for en value) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2360u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_en_value_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..en..value (descendant search for en value) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en..value")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2360u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_mainsnak_datavalue_value_id_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..mainsnak..datavalue..value..id (descendant search for mainsnak datavalue value id) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..mainsnak..datavalue..value..id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 12958u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_mainsnak_datavalue_value_id_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..mainsnak..datavalue..value..id (descendant search for mainsnak datavalue value id) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..mainsnak..datavalue..value..id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 12958u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_mainsnak_datavalue_value_id_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..mainsnak..datavalue..value..id (descendant search for mainsnak datavalue value id) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..mainsnak..datavalue..value..id")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 12958u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_mainsnak_datavalue_value_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..mainsnak..datavalue..value (descendant search for mainsnak datavalue value) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..mainsnak..datavalue..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 26115u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_mainsnak_datavalue_value_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..mainsnak..datavalue..value (descendant search for mainsnak datavalue value) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..mainsnak..datavalue..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 26115u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_mainsnak_datavalue_value_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..mainsnak..datavalue..value (descendant search for mainsnak datavalue value) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..mainsnak..datavalue..value")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 26115u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_references_snaks_datavalue_value_id_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..references..snaks..datavalue..value..id (descendant search for references snaks datavalue value id) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..references..snaks..datavalue..value..id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11113u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_references_snaks_datavalue_value_id_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..references..snaks..datavalue..value..id (descendant search for references snaks datavalue value id) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..references..snaks..datavalue..value..id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11113u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_references_snaks_datavalue_value_id_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..references..snaks..datavalue..value..id (descendant search for references snaks datavalue value id) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..references..snaks..datavalue..value..id")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11113u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_references_snaks_datavalue_value_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..references..snaks..datavalue..value (descendant search for references snaks datavalue value) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..references..snaks..datavalue..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25118u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_references_snaks_datavalue_value_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..references..snaks..datavalue..value (descendant search for references snaks datavalue value) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..references..snaks..datavalue..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25118u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_references_snaks_datavalue_value_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..references..snaks..datavalue..value (descendant search for references snaks datavalue value) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..references..snaks..datavalue..value")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25118u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_references_snaks_datavalue_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..references..snaks..datavalue (descendant search for references snaks datavalue) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..references..snaks..datavalue")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25118u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_references_snaks_datavalue_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..references..snaks..datavalue (descendant search for references snaks datavalue) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..references..snaks..datavalue")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25118u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_references_snaks_datavalue_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..references..snaks..datavalue (descendant search for references snaks datavalue) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..references..snaks..datavalue")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25118u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_snaks_datavalue_value_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..snaks..datavalue..value (descendant search for snaks datavalue value) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..snaks..datavalue..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25118u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_snaks_datavalue_value_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..snaks..datavalue..value (descendant search for snaks datavalue value) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..snaks..datavalue..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25118u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_snaks_datavalue_value_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..snaks..datavalue..value (descendant search for snaks datavalue value) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..snaks..datavalue..value")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 25118u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_snaks_then_any_descendant_and_its_id_child_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..snaks..*.id (descendant search for snaks, then any descendant and its id child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..snaks..*.id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11113u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_snaks_then_any_descendant_and_its_id_child_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..snaks..*.id (descendant search for snaks, then any descendant and its id child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..snaks..*.id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11113u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_descendant_search_for_snaks_then_any_descendant_and_its_id_child_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..snaks..*.id (descendant search for snaks, then any descendant and its id child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..snaks..*.id")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11113u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_select_all_nodes_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..* (select all nodes) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 970442u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_select_all_nodes_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..* (select all nodes) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 970442u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_person_with_query_select_all_nodes_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_person.toml running the query $..* (select all nodes) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_person.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 970442u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_compressed_with_query_all_nodes_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_profession.toml running the query $..* (all nodes) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_profession.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1702482u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_compressed_with_query_all_nodes_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_profession.toml running the query $..* (all nodes) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_profession.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1702482u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_compressed_with_query_all_nodes_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_profession.toml running the query $..* (all nodes) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json =
        fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_profession.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1702482u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_compressed_with_query_descendant_search_for_any_node_and_then_its_id_child_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_profession.toml running the query $..*.id (descendant search for any node and then its 'id' child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_profession.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 98805u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_compressed_with_query_descendant_search_for_any_node_and_then_its_id_child_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_profession.toml running the query $..*.id (descendant search for any node and then its 'id' child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_profession.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 98805u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_compressed_with_query_descendant_search_for_any_node_and_then_its_id_child_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_profession.toml running the query $..*.id (descendant search for any node and then its 'id' child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.id")?;
    let raw_json =
        fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_profession.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 98805u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_compressed_with_query_descendant_search_for_claims_mainsnak_value_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_profession.toml running the query $..claims..mainsnak..value (descendant search for claims mainsnak value) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..claims..mainsnak..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_profession.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 59112u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_compressed_with_query_descendant_search_for_claims_mainsnak_value_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_profession.toml running the query $..claims..mainsnak..value (descendant search for claims mainsnak value) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..claims..mainsnak..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_profession.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 59112u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_compressed_with_query_descendant_search_for_claims_mainsnak_value_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_profession.toml running the query $..claims..mainsnak..value (descendant search for claims mainsnak value) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..claims..mainsnak..value")?;
    let raw_json =
        fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_profession.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 59112u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_compressed_with_query_descendant_search_for_en_then_its_value_child_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_profession.toml running the query $..en.value (descendant search for en, then its 'value' child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en.value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_profession.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9452u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_compressed_with_query_descendant_search_for_en_then_its_value_child_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_profession.toml running the query $..en.value (descendant search for en, then its 'value' child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en.value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_profession.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9452u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_compressed_with_query_descendant_search_for_en_then_its_value_child_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_profession.toml running the query $..en.value (descendant search for en, then its 'value' child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en.value")?;
    let raw_json =
        fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_profession.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9452u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_compressed_with_query_descendant_search_for_en_value_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_profession.toml running the query $..en..value (descendant search for en value) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_profession.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 13634u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_compressed_with_query_descendant_search_for_en_value_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_profession.toml running the query $..en..value (descendant search for en value) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_profession.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 13634u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_compressed_with_query_descendant_search_for_en_value_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_profession.toml running the query $..en..value (descendant search for en value) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en..value")?;
    let raw_json =
        fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_profession.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 13634u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_with_query_all_nodes_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_profession.toml running the query $..* (all nodes) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_profession.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1702482u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_with_query_all_nodes_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_profession.toml running the query $..* (all nodes) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_profession.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1702482u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_with_query_all_nodes_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_profession.toml running the query $..* (all nodes) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_profession.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1702482u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_with_query_descendant_search_for_any_node_and_then_its_id_child_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_profession.toml running the query $..*.id (descendant search for any node and then its 'id' child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_profession.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 98805u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_with_query_descendant_search_for_any_node_and_then_its_id_child_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_profession.toml running the query $..*.id (descendant search for any node and then its 'id' child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_profession.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 98805u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_with_query_descendant_search_for_any_node_and_then_its_id_child_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_profession.toml running the query $..*.id (descendant search for any node and then its 'id' child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.id")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_profession.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 98805u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_with_query_descendant_search_for_claims_mainsnak_value_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_profession.toml running the query $..claims..mainsnak..value (descendant search for claims mainsnak value) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..claims..mainsnak..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_profession.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 59112u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_with_query_descendant_search_for_claims_mainsnak_value_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_profession.toml running the query $..claims..mainsnak..value (descendant search for claims mainsnak value) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..claims..mainsnak..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_profession.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 59112u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_with_query_descendant_search_for_claims_mainsnak_value_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_profession.toml running the query $..claims..mainsnak..value (descendant search for claims mainsnak value) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..claims..mainsnak..value")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_profession.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 59112u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_with_query_descendant_search_for_en_then_its_value_child_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_profession.toml running the query $..en.value (descendant search for en, then its 'value' child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en.value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_profession.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9452u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_with_query_descendant_search_for_en_then_its_value_child_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_profession.toml running the query $..en.value (descendant search for en, then its 'value' child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en.value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_profession.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9452u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_with_query_descendant_search_for_en_then_its_value_child_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_profession.toml running the query $..en.value (descendant search for en, then its 'value' child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en.value")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_profession.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 9452u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_with_query_descendant_search_for_en_value_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_profession.toml running the query $..en..value (descendant search for en value) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_profession.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 13634u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_with_query_descendant_search_for_en_value_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_profession.toml running the query $..en..value (descendant search for en value) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_profession.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 13634u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_profession_with_query_descendant_search_for_en_value_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_profession.toml running the query $..en..value (descendant search for en value) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en..value")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_profession.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 13634u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_all_nodes_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..* (all nodes) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 922519u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_all_nodes_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..* (all nodes) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 922519u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_all_nodes_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..* (all nodes) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json =
        fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 922519u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_descendant_search_for_any_node_and_then_its_value_child_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..*.value (descendant search for any node and then its 'value' child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 132188u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_descendant_search_for_any_node_and_then_its_value_child_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..*.value (descendant search for any node and then its 'value' child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 132188u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_descendant_search_for_any_node_and_then_its_value_child_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..*.value (descendant search for any node and then its 'value' child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.value")?;
    let raw_json =
        fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 132188u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_descendant_search_for_en_and_then_its_value_child_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..en.value (descendant search for en, and then its 'value' child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en.value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1760u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_descendant_search_for_en_and_then_its_value_child_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..en.value (descendant search for en, and then its 'value' child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en.value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1760u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_descendant_search_for_en_and_then_its_value_child_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..en.value (descendant search for en, and then its 'value' child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en.value")?;
    let raw_json =
        fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1760u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_descendant_search_for_en_value_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..en..value (descendant search for en value) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4504u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_descendant_search_for_en_value_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..en..value (descendant search for en value) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4504u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_descendant_search_for_en_value_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..en..value (descendant search for en value) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en..value")?;
    let raw_json =
        fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4504u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_descendant_search_for_qualifiers_datavalue_id_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..qualifiers..datavalue..id (descendant search for qualifiers datavalue id) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..qualifiers..datavalue..id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 18219u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_descendant_search_for_qualifiers_datavalue_id_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..qualifiers..datavalue..id (descendant search for qualifiers datavalue id) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..qualifiers..datavalue..id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 18219u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_descendant_search_for_qualifiers_datavalue_id_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..qualifiers..datavalue..id (descendant search for qualifiers datavalue id) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..qualifiers..datavalue..id")?;
    let raw_json =
        fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 18219u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_descendant_search_for_the_fifth_array_element_of_any_node_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..*[5] (descendant search for the fifth array element of any node) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[5]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2511u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_descendant_search_for_the_fifth_array_element_of_any_node_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..*[5] (descendant search for the fifth array element of any node) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[5]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2511u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_descendant_search_for_the_fifth_array_element_of_any_node_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..*[5] (descendant search for the fifth array element of any node) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[5]")?;
    let raw_json =
        fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2511u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_path_to_p7103_claims_p31_references_snaks_p4656_hash_with_descendants_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..P7103.claims.P31..references..snaks.P4656..hash (path to P7103 claims P31 references snaks P4656 hash with descendants) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..P7103.claims.P31..references..snaks.P4656..hash")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_path_to_p7103_claims_p31_references_snaks_p4656_hash_with_descendants_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..P7103.claims.P31..references..snaks.P4656..hash (path to P7103 claims P31 references snaks P4656 hash with descendants) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..P7103.claims.P31..references..snaks.P4656..hash")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21204179usize,], "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_path_to_p7103_claims_p31_references_snaks_p4656_hash_with_descendants_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..P7103.claims.P31..references..snaks.P4656..hash (path to P7103 claims P31 references snaks P4656 hash with descendants) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..P7103.claims.P31..references..snaks.P4656..hash")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_path_to_p7103_claims_p31_references_snaks_p4656_hash_with_descendants_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..P7103.claims.P31..references..snaks.P4656..hash (path to P7103 claims P31 references snaks P4656 hash with descendants) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..P7103.claims.P31..references..snaks.P4656..hash")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21204179usize,], "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_path_to_p7103_claims_p31_references_snaks_p4656_hash_with_descendants_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..P7103.claims.P31..references..snaks.P4656..hash (path to P7103 claims P31 references snaks P4656 hash with descendants) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..P7103.claims.P31..references..snaks.P4656..hash")?;
    let raw_json =
        fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_compressed_with_query_path_to_p7103_claims_p31_references_snaks_p4656_hash_with_descendants_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/wikidata_properties.toml running the query $..P7103.claims.P31..references..snaks.P4656..hash (path to P7103 claims P31 references snaks P4656 hash with descendants) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..P7103.claims.P31..references..snaks.P4656..hash")?;
    let raw_json =
        fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/wikidata_properties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21204179usize,], "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_all_nodes_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..* (all nodes) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 922519u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_all_nodes_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..* (all nodes) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 922519u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_all_nodes_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..* (all nodes) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 922519u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_descendant_search_for_any_node_and_then_its_value_child_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..*.value (descendant search for any node and then its 'value' child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 132188u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_descendant_search_for_any_node_and_then_its_value_child_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..*.value (descendant search for any node and then its 'value' child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 132188u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_descendant_search_for_any_node_and_then_its_value_child_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..*.value (descendant search for any node and then its 'value' child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.value")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 132188u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_descendant_search_for_en_and_then_its_value_child_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..en.value (descendant search for en, and then its 'value' child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en.value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1760u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_descendant_search_for_en_and_then_its_value_child_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..en.value (descendant search for en, and then its 'value' child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en.value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1760u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_descendant_search_for_en_and_then_its_value_child_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..en.value (descendant search for en, and then its 'value' child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en.value")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1760u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_descendant_search_for_en_value_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..en..value (descendant search for en value) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4504u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_descendant_search_for_en_value_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..en..value (descendant search for en value) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en..value")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4504u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_descendant_search_for_en_value_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..en..value (descendant search for en value) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..en..value")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4504u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_descendant_search_for_qualifiers_datavalue_id_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..qualifiers..datavalue..id (descendant search for qualifiers datavalue id) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..qualifiers..datavalue..id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 18219u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_descendant_search_for_qualifiers_datavalue_id_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..qualifiers..datavalue..id (descendant search for qualifiers datavalue id) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..qualifiers..datavalue..id")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 18219u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_descendant_search_for_qualifiers_datavalue_id_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..qualifiers..datavalue..id (descendant search for qualifiers datavalue id) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..qualifiers..datavalue..id")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 18219u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_descendant_search_for_the_fifth_array_element_of_any_node_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..*[5] (descendant search for the fifth array element of any node) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[5]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2511u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_descendant_search_for_the_fifth_array_element_of_any_node_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..*[5] (descendant search for the fifth array element of any node) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[5]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2511u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_descendant_search_for_the_fifth_array_element_of_any_node_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..*[5] (descendant search for the fifth array element of any node) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[5]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2511u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_path_to_p7103_claims_p31_references_snaks_p4656_hash_with_descendants_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..P7103.claims.P31..references..snaks.P4656..hash (path to P7103 claims P31 references snaks P4656 hash with descendants) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..P7103.claims.P31..references..snaks.P4656..hash")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_path_to_p7103_claims_p31_references_snaks_p4656_hash_with_descendants_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..P7103.claims.P31..references..snaks.P4656..hash (path to P7103 claims P31 references snaks P4656 hash with descendants) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..P7103.claims.P31..references..snaks.P4656..hash")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![22639035usize,], "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_path_to_p7103_claims_p31_references_snaks_p4656_hash_with_descendants_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..P7103.claims.P31..references..snaks.P4656..hash (path to P7103 claims P31 references snaks P4656 hash with descendants) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..P7103.claims.P31..references..snaks.P4656..hash")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_path_to_p7103_claims_p31_references_snaks_p4656_hash_with_descendants_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..P7103.claims.P31..references..snaks.P4656..hash (path to P7103 claims P31 references snaks P4656 hash with descendants) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..P7103.claims.P31..references..snaks.P4656..hash")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![22639035usize,], "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_path_to_p7103_claims_p31_references_snaks_p4656_hash_with_descendants_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..P7103.claims.P31..references..snaks.P4656..hash (path to P7103 claims P31 references snaks P4656 hash with descendants) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..P7103.claims.P31..references..snaks.P4656..hash")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn large_wikidata_dump_properties_with_query_path_to_p7103_claims_p31_references_snaks_p4656_hash_with_descendants_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document wikidata_properties.toml running the query $..P7103.claims.P31..references..snaks.P4656..hash (path to P7103 claims P31 references snaks P4656 hash with descendants) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..P7103.claims.P31..references..snaks.P4656..hash")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/wikidata_properties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![22639035usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_integers_and_objects_compressed_with_query_select_all_elements_on_the_list_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_integers_and_objects_compressed_with_query_select_all_elements_on_the_list_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize, 9usize, 18usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_integers_and_objects_compressed_with_query_select_all_elements_on_the_list_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_integers_and_objects_compressed_with_query_select_all_elements_on_the_list_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize, 9usize, 18usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_integers_and_objects_compressed_with_query_select_all_elements_on_the_list_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_integers_and_objects_compressed_with_query_select_all_elements_on_the_list_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize, 9usize, 18usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_integers_and_objects_with_query_select_all_elements_on_the_list_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_integers_and_objects_with_query_select_all_elements_on_the_list_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/heterogenous_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![15usize, 23usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_integers_and_objects_with_query_select_all_elements_on_the_list_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_integers_and_objects_with_query_select_all_elements_on_the_list_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/heterogenous_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![15usize, 23usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_integers_and_objects_with_query_select_all_elements_on_the_list_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn list_with_mixed_atomic_integers_and_objects_with_query_select_all_elements_on_the_list_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document heterogenous_list.toml running the query $.a.* (select all elements on the list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/heterogenous_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![15usize, 23usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_compressed_with_query_select_label_with_one_actual_backslash_which_is_two_backslashes_in_the_query_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/escapes.toml running the query $..a..b..['label\\\\'] (select label with one actual backslash, which is two backslashes in the query) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_compressed_with_query_select_label_with_one_actual_backslash_which_is_two_backslashes_in_the_query_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/escapes.toml running the query $..a..b..['label\\\\'] (select label with one actual backslash, which is two backslashes in the query) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![525usize,], "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_compressed_with_query_select_label_with_one_actual_backslash_which_is_two_backslashes_in_the_query_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/escapes.toml running the query $..a..b..['label\\\\'] (select label with one actual backslash, which is two backslashes in the query) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_compressed_with_query_select_label_with_one_actual_backslash_which_is_two_backslashes_in_the_query_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/escapes.toml running the query $..a..b..['label\\\\'] (select label with one actual backslash, which is two backslashes in the query) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![525usize,], "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_compressed_with_query_select_label_with_one_actual_backslash_which_is_two_backslashes_in_the_query_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/escapes.toml running the query $..a..b..['label\\\\'] (select label with one actual backslash, which is two backslashes in the query) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_compressed_with_query_select_label_with_one_actual_backslash_which_is_two_backslashes_in_the_query_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/escapes.toml running the query $..a..b..['label\\\\'] (select label with one actual backslash, which is two backslashes in the query) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![525usize,], "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_compressed_with_query_select_label_with_two_actual_backslashes_four_backslashes_in_the_query_which_does_not_exist_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/escapes.toml running the query $..a..b..['label\\\\\\\\'] (select label with two actual backslashes (four backslashes in the query), which does not exist) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_compressed_with_query_select_label_with_two_actual_backslashes_four_backslashes_in_the_query_which_does_not_exist_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/escapes.toml running the query $..a..b..['label\\\\\\\\'] (select label with two actual backslashes (four backslashes in the query), which does not exist) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_compressed_with_query_select_label_with_two_actual_backslashes_four_backslashes_in_the_query_which_does_not_exist_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/escapes.toml running the query $..a..b..['label\\\\\\\\'] (select label with two actual backslashes (four backslashes in the query), which does not exist) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_compressed_with_query_select_label_with_two_actual_backslashes_four_backslashes_in_the_query_which_does_not_exist_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/escapes.toml running the query $..a..b..['label\\\\\\\\'] (select label with two actual backslashes (four backslashes in the query), which does not exist) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_compressed_with_query_select_label_with_two_actual_backslashes_four_backslashes_in_the_query_which_does_not_exist_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/escapes.toml running the query $..a..b..['label\\\\\\\\'] (select label with two actual backslashes (four backslashes in the query), which does not exist) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_compressed_with_query_select_label_with_two_actual_backslashes_four_backslashes_in_the_query_which_does_not_exist_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/escapes.toml running the query $..a..b..['label\\\\\\\\'] (select label with two actual backslashes (four backslashes in the query), which does not exist) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_with_query_select_label_with_one_actual_backslash_which_is_two_backslashes_in_the_query_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\'] (select label with one actual backslash, which is two backslashes in the query) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_with_query_select_label_with_one_actual_backslash_which_is_two_backslashes_in_the_query_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\'] (select label with one actual backslash, which is two backslashes in the query) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![611usize,], "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_with_query_select_label_with_one_actual_backslash_which_is_two_backslashes_in_the_query_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\'] (select label with one actual backslash, which is two backslashes in the query) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_with_query_select_label_with_one_actual_backslash_which_is_two_backslashes_in_the_query_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\'] (select label with one actual backslash, which is two backslashes in the query) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![611usize,], "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_with_query_select_label_with_one_actual_backslash_which_is_two_backslashes_in_the_query_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\'] (select label with one actual backslash, which is two backslashes in the query) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_with_query_select_label_with_one_actual_backslash_which_is_two_backslashes_in_the_query_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\'] (select label with one actual backslash, which is two backslashes in the query) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\']")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![611usize,], "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_with_query_select_label_with_two_actual_backslashes_four_backslashes_in_the_query_which_does_not_exist_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\\\\\'] (select label with two actual backslashes (four backslashes in the query), which does not exist) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_with_query_select_label_with_two_actual_backslashes_four_backslashes_in_the_query_which_does_not_exist_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\\\\\'] (select label with two actual backslashes (four backslashes in the query), which does not exist) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/escapes.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_with_query_select_label_with_two_actual_backslashes_four_backslashes_in_the_query_which_does_not_exist_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\\\\\'] (select label with two actual backslashes (four backslashes in the query), which does not exist) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_with_query_select_label_with_two_actual_backslashes_four_backslashes_in_the_query_which_does_not_exist_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\\\\\'] (select label with two actual backslashes (four backslashes in the query), which does not exist) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/escapes.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_with_query_select_label_with_two_actual_backslashes_four_backslashes_in_the_query_which_does_not_exist_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\\\\\'] (select label with two actual backslashes (four backslashes in the query), which does not exist) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn members_with_escaped_double_quotes_and_braces_and_brackets_with_query_select_label_with_two_actual_backslashes_four_backslashes_in_the_query_which_does_not_exist_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document escapes.toml running the query $..a..b..['label\\\\\\\\'] (select label with two actual backslashes (four backslashes in the query), which does not exist) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b..['label\\\\\\\\']")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/escapes.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_look_for_b_on_at_least_one_level_of_nesting_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_look_for_b_on_at_least_one_level_of_nesting_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![18usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_look_for_b_on_at_least_one_level_of_nesting_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_look_for_b_on_at_least_one_level_of_nesting_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![18usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_look_for_b_on_at_least_one_level_of_nesting_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_look_for_b_on_at_least_one_level_of_nesting_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![18usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_look_for_descendants_of_an_atomic_value_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_look_for_descendants_of_an_atomic_value_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_look_for_descendants_of_an_atomic_value_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_look_for_descendants_of_an_atomic_value_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_look_for_descendants_of_an_atomic_value_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_look_for_descendants_of_an_atomic_value_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_a_number_that_is_a_child_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_a_number_that_is_a_child_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_a_number_that_is_a_child_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_a_number_that_is_a_child_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_a_number_that_is_a_child_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_a_number_that_is_a_child_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_all_decsendants_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..* (select all decsendants) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_all_decsendants_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..* (select all decsendants) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![5usize, 12usize, 13usize, 18usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_all_decsendants_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..* (select all decsendants) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_all_decsendants_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..* (select all decsendants) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![5usize, 12usize, 13usize, 18usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_all_decsendants_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..* (select all decsendants) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_all_decsendants_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..* (select all decsendants) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![5usize, 12usize, 13usize, 18usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_first_item_from_list_descendants_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_first_item_from_list_descendants_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![13usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_first_item_from_list_descendants_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_first_item_from_list_descendants_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![13usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_first_item_from_list_descendants_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_first_item_from_list_descendants_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![13usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_first_element_of_b_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $.b[0] (select the first element of 'b') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_first_element_of_b_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $.b[0] (select the first element of 'b') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![13usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_first_element_of_b_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $.b[0] (select the first element of 'b') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_first_element_of_b_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $.b[0] (select the first element of 'b') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![13usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_first_element_of_b_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $.b[0] (select the first element of 'b') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_first_element_of_b_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $.b[0] (select the first element of 'b') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![13usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_nested_b_directly_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $.b[0].b (select the nested 'b' directly) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0].b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_nested_b_directly_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $.b[0].b (select the nested 'b' directly) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0].b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![18usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_nested_b_directly_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $.b[0].b (select the nested 'b' directly) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0].b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_nested_b_directly_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $.b[0].b (select the nested 'b' directly) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0].b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![18usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_nested_b_directly_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $.b[0].b (select the nested 'b' directly) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0].b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_nested_b_directly_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $.b[0].b (select the nested 'b' directly) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0].b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![18usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_second_element_of_b_which_does_not_exist_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $.b[1] (select the second element of 'b' (which does not exist)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_second_element_of_b_which_does_not_exist_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $.b[1] (select the second element of 'b' (which does not exist)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_second_element_of_b_which_does_not_exist_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $.b[1] (select the second element of 'b' (which does not exist)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_second_element_of_b_which_does_not_exist_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $.b[1] (select the second element of 'b' (which does not exist)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_second_element_of_b_which_does_not_exist_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $.b[1] (select the second element of 'b' (which does not exist)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_second_element_of_b_which_does_not_exist_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $.b[1] (select the second element of 'b' (which does not exist)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_second_element_of_the_root_which_is_not_an_array_so_result_should_be_empty_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $[1] (select the second element of the root (which is not an array, so result should be empty)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_second_element_of_the_root_which_is_not_an_array_so_result_should_be_empty_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $[1] (select the second element of the root (which is not an array, so result should be empty)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_second_element_of_the_root_which_is_not_an_array_so_result_should_be_empty_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $[1] (select the second element of the root (which is not an array, so result should be empty)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_second_element_of_the_root_which_is_not_an_array_so_result_should_be_empty_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $[1] (select the second element of the root (which is not an array, so result should be empty)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_second_element_of_the_root_which_is_not_an_array_so_result_should_be_empty_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $[1] (select the second element of the root (which is not an array, so result should be empty)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_compressed_with_query_select_the_second_element_of_the_root_which_is_not_an_array_so_result_should_be_empty_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_descendant.toml running the query $[1] (select the second element of the root (which is not an array, so result should be empty)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_look_for_b_on_at_least_one_level_of_nesting_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_look_for_b_on_at_least_one_level_of_nesting_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![53usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_look_for_b_on_at_least_one_level_of_nesting_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_look_for_b_on_at_least_one_level_of_nesting_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![53usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_look_for_b_on_at_least_one_level_of_nesting_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_look_for_b_on_at_least_one_level_of_nesting_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..*..b (look for 'b' on at least one level of nesting) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![53usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_look_for_descendants_of_an_atomic_value_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_look_for_descendants_of_an_atomic_value_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_look_for_descendants_of_an_atomic_value_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_look_for_descendants_of_an_atomic_value_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_look_for_descendants_of_an_atomic_value_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_look_for_descendants_of_an_atomic_value_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a..b (look for descendants of an atomic value) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_a_number_that_is_a_child_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_a_number_that_is_a_child_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![11usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_a_number_that_is_a_child_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_a_number_that_is_a_child_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![11usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_a_number_that_is_a_child_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_a_number_that_is_a_child_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..a (select a number that is a child) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![11usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_all_decsendants_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_all_decsendants_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![11usize, 24usize, 34usize, 53usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_all_decsendants_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_all_decsendants_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![11usize, 24usize, 34usize, 53usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_all_decsendants_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_all_decsendants_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..* (select all decsendants) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![11usize, 24usize, 34usize, 53usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_first_item_from_list_descendants_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_first_item_from_list_descendants_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![34usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_first_item_from_list_descendants_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_first_item_from_list_descendants_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![34usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_first_item_from_list_descendants_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_first_item_from_list_descendants_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $..[0] (select first item from list descendants) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![34usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_first_element_of_b_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $.b[0] (select the first element of 'b') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_first_element_of_b_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $.b[0] (select the first element of 'b') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![34usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_first_element_of_b_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $.b[0] (select the first element of 'b') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_first_element_of_b_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $.b[0] (select the first element of 'b') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![34usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_first_element_of_b_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $.b[0] (select the first element of 'b') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_first_element_of_b_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $.b[0] (select the first element of 'b') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![34usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_nested_b_directly_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $.b[0].b (select the nested 'b' directly) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0].b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_nested_b_directly_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $.b[0].b (select the nested 'b' directly) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0].b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![53usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_nested_b_directly_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $.b[0].b (select the nested 'b' directly) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0].b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_nested_b_directly_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $.b[0].b (select the nested 'b' directly) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0].b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![53usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_nested_b_directly_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $.b[0].b (select the nested 'b' directly) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0].b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_nested_b_directly_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $.b[0].b (select the nested 'b' directly) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[0].b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![53usize,], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_second_element_of_b_which_does_not_exist_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $.b[1] (select the second element of 'b' (which does not exist)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_second_element_of_b_which_does_not_exist_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $.b[1] (select the second element of 'b' (which does not exist)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_second_element_of_b_which_does_not_exist_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $.b[1] (select the second element of 'b' (which does not exist)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_second_element_of_b_which_does_not_exist_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $.b[1] (select the second element of 'b' (which does not exist)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_second_element_of_b_which_does_not_exist_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $.b[1] (select the second element of 'b' (which does not exist)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_second_element_of_b_which_does_not_exist_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $.b[1] (select the second element of 'b' (which does not exist)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.b[1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_second_element_of_the_root_which_is_not_an_array_so_result_should_be_empty_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $[1] (select the second element of the root (which is not an array, so result should be empty)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_second_element_of_the_root_which_is_not_an_array_so_result_should_be_empty_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $[1] (select the second element of the root (which is not an array, so result should be empty)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_second_element_of_the_root_which_is_not_an_array_so_result_should_be_empty_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $[1] (select the second element of the root (which is not an array, so result should be empty)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_second_element_of_the_root_which_is_not_an_array_so_result_should_be_empty_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $[1] (select the second element of the root (which is not an array, so result should be empty)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_second_element_of_the_root_which_is_not_an_array_so_result_should_be_empty_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $[1] (select the second element of the root (which is not an array, so result should be empty)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn nested_atomic_member_with_query_select_the_second_element_of_the_root_which_is_not_an_array_so_result_should_be_empty_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_descendant.toml running the query $[1] (select the second element of the root (which is not an array, so result should be empty)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_descendant.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_a_object_and_then_the_atomic_integer_by_descendant_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a..b (select the 'a' object and then the atomic integer by descendant) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_a_object_and_then_the_atomic_integer_by_descendant_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a..b (select the 'a' object and then the atomic integer by descendant) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![31usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_a_object_and_then_the_atomic_integer_by_descendant_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a..b (select the 'a' object and then the atomic integer by descendant) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_a_object_and_then_the_atomic_integer_by_descendant_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a..b (select the 'a' object and then the atomic integer by descendant) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![31usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_a_object_and_then_the_atomic_integer_by_descendant_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a..b (select the 'a' object and then the atomic integer by descendant) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_a_object_and_then_the_atomic_integer_by_descendant_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a..b (select the 'a' object and then the atomic integer by descendant) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![31usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_atomic_integer_by_descendant_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $..b (select the atomic integer by descendant) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_atomic_integer_by_descendant_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $..b (select the atomic integer by descendant) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![31usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_atomic_integer_by_descendant_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $..b (select the atomic integer by descendant) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_atomic_integer_by_descendant_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $..b (select the atomic integer by descendant) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![31usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_atomic_integer_by_descendant_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $..b (select the atomic integer by descendant) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_atomic_integer_by_descendant_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $..b (select the atomic integer by descendant) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![31usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_first_and_only_element_of_the_a_list_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a[0] (select the first and only element of the 'a' list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_first_and_only_element_of_the_a_list_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a[0] (select the first and only element of the 'a' list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_first_and_only_element_of_the_a_list_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a[0] (select the first and only element of the 'a' list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_first_and_only_element_of_the_a_list_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a[0] (select the first and only element of the 'a' list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_first_and_only_element_of_the_a_list_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a[0] (select the first and only element of the 'a' list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_first_and_only_element_of_the_a_list_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a[0] (select the first and only element of the 'a' list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_fourth_element_of_the_deeply_nested_list_which_does_not_exist_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a[0].c.d[3] (select the fourth element of the deeply nested list (which does not exist)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_fourth_element_of_the_deeply_nested_list_which_does_not_exist_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a[0].c.d[3] (select the fourth element of the deeply nested list (which does not exist)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_fourth_element_of_the_deeply_nested_list_which_does_not_exist_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a[0].c.d[3] (select the fourth element of the deeply nested list (which does not exist)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_fourth_element_of_the_deeply_nested_list_which_does_not_exist_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a[0].c.d[3] (select the fourth element of the deeply nested list (which does not exist)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_fourth_element_of_the_deeply_nested_list_which_does_not_exist_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a[0].c.d[3] (select the fourth element of the deeply nested list (which does not exist)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_fourth_element_of_the_deeply_nested_list_which_does_not_exist_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a[0].c.d[3] (select the fourth element of the deeply nested list (which does not exist)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_third_and_last_element_of_the_deeply_nested_list_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a[0].c.d[2] (select the third and last element of the deeply nested list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_third_and_last_element_of_the_deeply_nested_list_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a[0].c.d[2] (select the third and last element of the deeply nested list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![23usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_third_and_last_element_of_the_deeply_nested_list_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a[0].c.d[2] (select the third and last element of the deeply nested list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_third_and_last_element_of_the_deeply_nested_list_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a[0].c.d[2] (select the third and last element of the deeply nested list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![23usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_third_and_last_element_of_the_deeply_nested_list_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a[0].c.d[2] (select the third and last element of the deeply nested list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_third_and_last_element_of_the_deeply_nested_list_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $.a[0].c.d[2] (select the third and last element of the deeply nested list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![23usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_third_element_of_each_list_only_the_deeply_nested_one_has_a_third_element_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $..*[2] (select the third element of each list (only the deeply nested one has a third element)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_third_element_of_each_list_only_the_deeply_nested_one_has_a_third_element_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $..*[2] (select the third element of each list (only the deeply nested one has a third element)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![23usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_third_element_of_each_list_only_the_deeply_nested_one_has_a_third_element_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $..*[2] (select the third element of each list (only the deeply nested one has a third element)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_third_element_of_each_list_only_the_deeply_nested_one_has_a_third_element_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $..*[2] (select the third element of each list (only the deeply nested one has a third element)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![23usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_third_element_of_each_list_only_the_deeply_nested_one_has_a_third_element_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $..*[2] (select the third element of each list (only the deeply nested one has a third element)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_compressed_with_query_select_the_third_element_of_each_list_only_the_deeply_nested_one_has_a_third_element_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/atomic_after_list.toml running the query $..*[2] (select the third element of each list (only the deeply nested one has a third element)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![23usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_a_object_and_then_the_atomic_integer_by_descendant_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a..b (select the 'a' object and then the atomic integer by descendant) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_a_object_and_then_the_atomic_integer_by_descendant_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a..b (select the 'a' object and then the atomic integer by descendant) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![176usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_a_object_and_then_the_atomic_integer_by_descendant_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a..b (select the 'a' object and then the atomic integer by descendant) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_a_object_and_then_the_atomic_integer_by_descendant_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a..b (select the 'a' object and then the atomic integer by descendant) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![176usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_a_object_and_then_the_atomic_integer_by_descendant_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a..b (select the 'a' object and then the atomic integer by descendant) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_a_object_and_then_the_atomic_integer_by_descendant_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a..b (select the 'a' object and then the atomic integer by descendant) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![176usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_atomic_integer_by_descendant_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $..b (select the atomic integer by descendant) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_atomic_integer_by_descendant_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $..b (select the atomic integer by descendant) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![176usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_atomic_integer_by_descendant_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $..b (select the atomic integer by descendant) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_atomic_integer_by_descendant_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $..b (select the atomic integer by descendant) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![176usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_atomic_integer_by_descendant_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $..b (select the atomic integer by descendant) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_atomic_integer_by_descendant_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $..b (select the atomic integer by descendant) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..b")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![176usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_first_and_only_element_of_the_a_list_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a[0] (select the first and only element of the 'a' list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_first_and_only_element_of_the_a_list_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a[0] (select the first and only element of the 'a' list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_first_and_only_element_of_the_a_list_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a[0] (select the first and only element of the 'a' list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_first_and_only_element_of_the_a_list_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a[0] (select the first and only element of the 'a' list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_first_and_only_element_of_the_a_list_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a[0] (select the first and only element of the 'a' list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_first_and_only_element_of_the_a_list_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a[0] (select the first and only element of the 'a' list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_fourth_element_of_the_deeply_nested_list_which_does_not_exist_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a[0].c.d[3] (select the fourth element of the deeply nested list (which does not exist)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_fourth_element_of_the_deeply_nested_list_which_does_not_exist_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a[0].c.d[3] (select the fourth element of the deeply nested list (which does not exist)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_fourth_element_of_the_deeply_nested_list_which_does_not_exist_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a[0].c.d[3] (select the fourth element of the deeply nested list (which does not exist)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_fourth_element_of_the_deeply_nested_list_which_does_not_exist_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a[0].c.d[3] (select the fourth element of the deeply nested list (which does not exist)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_fourth_element_of_the_deeply_nested_list_which_does_not_exist_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a[0].c.d[3] (select the fourth element of the deeply nested list (which does not exist)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 0u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_fourth_element_of_the_deeply_nested_list_which_does_not_exist_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a[0].c.d[3] (select the fourth element of the deeply nested list (which does not exist)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[3]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_third_and_last_element_of_the_deeply_nested_list_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a[0].c.d[2] (select the third and last element of the deeply nested list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_third_and_last_element_of_the_deeply_nested_list_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a[0].c.d[2] (select the third and last element of the deeply nested list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![133usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_third_and_last_element_of_the_deeply_nested_list_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a[0].c.d[2] (select the third and last element of the deeply nested list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_third_and_last_element_of_the_deeply_nested_list_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a[0].c.d[2] (select the third and last element of the deeply nested list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![133usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_third_and_last_element_of_the_deeply_nested_list_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a[0].c.d[2] (select the third and last element of the deeply nested list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_third_and_last_element_of_the_deeply_nested_list_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $.a[0].c.d[2] (select the third and last element of the deeply nested list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[0].c.d[2]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![133usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_third_element_of_each_list_only_the_deeply_nested_one_has_a_third_element_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $..*[2] (select the third element of each list (only the deeply nested one has a third element)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_third_element_of_each_list_only_the_deeply_nested_one_has_a_third_element_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $..*[2] (select the third element of each list (only the deeply nested one has a third element)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![133usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_third_element_of_each_list_only_the_deeply_nested_one_has_a_third_element_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $..*[2] (select the third element of each list (only the deeply nested one has a third element)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_third_element_of_each_list_only_the_deeply_nested_one_has_a_third_element_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $..*[2] (select the third element of each list (only the deeply nested one has a third element)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![133usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_third_element_of_each_list_only_the_deeply_nested_one_has_a_third_element_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $..*[2] (select the third element of each list (only the deeply nested one has a third element)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_a_list_of_integers_followed_by_an_atomic_integer_member_with_query_select_the_third_element_of_each_list_only_the_deeply_nested_one_has_a_third_element_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document atomic_after_list.toml running the query $..*[2] (select the third element of each list (only the deeply nested one has a third element)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[2]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/atomic_after_list.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![133usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_directly_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_directly_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![14usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_directly_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_directly_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![14usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_directly_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_directly_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![14usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_with_quote_directly_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_with_quote_directly_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![7usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_with_quote_directly_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_with_quote_directly_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![7usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_with_quote_directly_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_compressed_with_query_select_x_with_quote_directly_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![7usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_directly_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_directly_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![26usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_directly_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_directly_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![26usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_directly_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_directly_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['x'] (select 'x' directly) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['x']")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![26usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_with_quote_directly_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_with_quote_directly_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/quote_escape.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![13usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_with_quote_directly_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_with_quote_directly_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/quote_escape.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![13usize,], "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_with_quote_directly_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn object_with_two_labels_x_and_x_with_a_preceding_escaped_double_quote_with_query_select_x_with_quote_directly_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document quote_escape.toml running the query $['\"x'] (select 'x' with quote directly) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$['\"x']")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/quote_escape.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![13usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_direct_path_2_1_0_1_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1][0][1] (direct path 2-1-0-1) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_direct_path_2_1_0_1_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1][0][1] (direct path 2-1-0-1) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![19usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_direct_path_2_1_0_1_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1][0][1] (direct path 2-1-0-1) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_direct_path_2_1_0_1_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1][0][1] (direct path 2-1-0-1) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![19usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_direct_path_2_1_0_1_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1][0][1] (direct path 2-1-0-1) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_direct_path_2_1_0_1_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1][0][1] (direct path 2-1-0-1) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![19usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_direct_path_2_1_and_then_any_child_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1].* (direct path 2-1 and then any child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_direct_path_2_1_and_then_any_child_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1].* (direct path 2-1 and then any child) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![15usize, 22usize, 29usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_direct_path_2_1_and_then_any_child_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1].* (direct path 2-1 and then any child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_direct_path_2_1_and_then_any_child_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1].* (direct path 2-1 and then any child) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![15usize, 22usize, 29usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_direct_path_2_1_and_then_any_child_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1].* (direct path 2-1 and then any child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_direct_path_2_1_and_then_any_child_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1].* (direct path 2-1 and then any child) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![15usize, 22usize, 29usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_all_nodes_at_depth_one_and_then_their_first_list_element_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $.*[0] (select all nodes at depth one and then their first list element) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_all_nodes_at_depth_one_and_then_their_first_list_element_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $.*[0] (select all nodes at depth one and then their first list element) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![2usize, 11usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_all_nodes_at_depth_one_and_then_their_first_list_element_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $.*[0] (select all nodes at depth one and then their first list element) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_all_nodes_at_depth_one_and_then_their_first_list_element_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $.*[0] (select all nodes at depth one and then their first list element) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![2usize, 11usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_all_nodes_at_depth_one_and_then_their_first_list_element_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $.*[0] (select all nodes at depth one and then their first list element) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_all_nodes_at_depth_one_and_then_their_first_list_element_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $.*[0] (select all nodes at depth one and then their first list element) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![2usize, 11usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_all_non_root_nodes_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $..* (select all non-root nodes) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 16u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_all_non_root_nodes_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $..* (select all non-root nodes) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            1usize, 2usize, 3usize, 7usize, 10usize, 11usize, 14usize, 15usize, 16usize, 19usize, 22usize, 23usize,
            26usize, 29usize, 30usize, 33usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_all_non_root_nodes_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $..* (select all non-root nodes) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 16u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_all_non_root_nodes_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $..* (select all non-root nodes) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            1usize, 2usize, 3usize, 7usize, 10usize, 11usize, 14usize, 15usize, 16usize, 19usize, 22usize, 23usize,
            26usize, 29usize, 30usize, 33usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_all_non_root_nodes_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $..* (select all non-root nodes) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 16u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_all_non_root_nodes_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $..* (select all non-root nodes) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            1usize, 2usize, 3usize, 7usize, 10usize, 11usize, 14usize, 15usize, 16usize, 19usize, 22usize, 23usize,
            26usize, 29usize, 30usize, 33usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_and_then_all_its_children_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[0].* (select the first element and then all its children) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_and_then_all_its_children_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[0].* (select the first element and then all its children) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![2usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_and_then_all_its_children_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[0].* (select the first element and then all its children) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_and_then_all_its_children_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[0].* (select the first element and then all its children) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![2usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_and_then_all_its_children_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[0].* (select the first element and then all its children) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_and_then_all_its_children_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[0].* (select the first element and then all its children) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![2usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_of_every_nested_list_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $..*[0] (select the first element of every nested list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_of_every_nested_list_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $..*[0] (select the first element of every nested list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![2usize, 3usize, 11usize, 15usize, 16usize, 23usize, 30usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_of_every_nested_list_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $..*[0] (select the first element of every nested list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_of_every_nested_list_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $..*[0] (select the first element of every nested list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![2usize, 3usize, 11usize, 15usize, 16usize, 23usize, 30usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_of_every_nested_list_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $..*[0] (select the first element of every nested list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_of_every_nested_list_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $..*[0] (select the first element of every nested list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![2usize, 3usize, 11usize, 15usize, 16usize, 23usize, 30usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_of_the_long_list_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][0] (select the first element of the long list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_of_the_long_list_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][0] (select the first element of the long list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![11usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_of_the_long_list_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][0] (select the first element of the long list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_of_the_long_list_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][0] (select the first element of the long list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![11usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_of_the_long_list_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][0] (select the first element of the long list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_of_the_long_list_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][0] (select the first element of the long list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![11usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[0] (select the first element) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[0] (select the first element) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![1usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[0] (select the first element) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[0] (select the first element) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![1usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[0] (select the first element) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_first_element_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[0] (select the first element) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![1usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_long_list_and_then_its_every_subdocument_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2]..* (select the long list and then its every subdocument) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_long_list_and_then_its_every_subdocument_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2]..* (select the long list and then its every subdocument) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![11usize, 14usize, 15usize, 16usize, 19usize, 22usize, 23usize, 26usize, 29usize, 30usize, 33usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_long_list_and_then_its_every_subdocument_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2]..* (select the long list and then its every subdocument) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_long_list_and_then_its_every_subdocument_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2]..* (select the long list and then its every subdocument) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![11usize, 14usize, 15usize, 16usize, 19usize, 22usize, 23usize, 26usize, 29usize, 30usize, 33usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_long_list_and_then_its_every_subdocument_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2]..* (select the long list and then its every subdocument) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_long_list_and_then_its_every_subdocument_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2]..* (select the long list and then its every subdocument) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![11usize, 14usize, 15usize, 16usize, 19usize, 22usize, 23usize, 26usize, 29usize, 30usize, 33usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_long_list_then_go_two_levels_down_and_select_the_second_element_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2].*.*[1] (select the long list, then go two levels down and select the second element) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*.*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_long_list_then_go_two_levels_down_and_select_the_second_element_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2].*.*[1] (select the long list, then go two levels down and select the second element) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*.*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![19usize, 26usize, 33usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_long_list_then_go_two_levels_down_and_select_the_second_element_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2].*.*[1] (select the long list, then go two levels down and select the second element) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*.*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_long_list_then_go_two_levels_down_and_select_the_second_element_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2].*.*[1] (select the long list, then go two levels down and select the second element) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*.*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![19usize, 26usize, 33usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_long_list_then_go_two_levels_down_and_select_the_second_element_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2].*.*[1] (select the long list, then go two levels down and select the second element) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*.*[1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_long_list_then_go_two_levels_down_and_select_the_second_element_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2].*.*[1] (select the long list, then go two levels down and select the second element) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*.*[1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![19usize, 26usize, 33usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_long_list_then_in_each_sublist_select_the_second_element_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2].*[1] (select the long list, then in each sublist select the second element) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_long_list_then_in_each_sublist_select_the_second_element_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2].*[1] (select the long list, then in each sublist select the second element) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![22usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_long_list_then_in_each_sublist_select_the_second_element_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2].*[1] (select the long list, then in each sublist select the second element) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_long_list_then_in_each_sublist_select_the_second_element_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2].*[1] (select the long list, then in each sublist select the second element) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![22usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_long_list_then_in_each_sublist_select_the_second_element_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2].*[1] (select the long list, then in each sublist select the second element) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_long_list_then_in_each_sublist_select_the_second_element_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2].*[1] (select the long list, then in each sublist select the second element) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![22usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_path_2_1_then_in_any_subtree_select_the_second_list_element_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1]..*[1] (select the path 2-1, then in any subtree select the second list element) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_path_2_1_then_in_any_subtree_select_the_second_list_element_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1]..*[1] (select the path 2-1, then in any subtree select the second list element) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![19usize, 26usize, 33usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_path_2_1_then_in_any_subtree_select_the_second_list_element_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1]..*[1] (select the path 2-1, then in any subtree select the second list element) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_path_2_1_then_in_any_subtree_select_the_second_list_element_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1]..*[1] (select the path 2-1, then in any subtree select the second list element) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![19usize, 26usize, 33usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_path_2_1_then_in_any_subtree_select_the_second_list_element_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1]..*[1] (select the path 2-1, then in any subtree select the second list element) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_path_2_1_then_in_any_subtree_select_the_second_list_element_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1]..*[1] (select the path 2-1, then in any subtree select the second list element) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![19usize, 26usize, 33usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_second_element_of_the_long_list_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1] (select the second element of the long list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_second_element_of_the_long_list_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1] (select the second element of the long list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![14usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_second_element_of_the_long_list_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1] (select the second element of the long list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_second_element_of_the_long_list_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1] (select the second element of the long list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![14usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_second_element_of_the_long_list_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1] (select the second element of the long list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_compressed_with_query_select_the_second_element_of_the_long_list_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/lists.toml running the query $[2][1] (select the second element of the long list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![14usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_direct_path_2_1_0_1_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1][0][1] (direct path 2-1-0-1) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_direct_path_2_1_0_1_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1][0][1] (direct path 2-1-0-1) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![113usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_direct_path_2_1_0_1_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1][0][1] (direct path 2-1-0-1) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_direct_path_2_1_0_1_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1][0][1] (direct path 2-1-0-1) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![113usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_direct_path_2_1_0_1_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1][0][1] (direct path 2-1-0-1) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_direct_path_2_1_0_1_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1][0][1] (direct path 2-1-0-1) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1][0][1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![113usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_direct_path_2_1_and_then_any_child_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1].* (direct path 2-1 and then any child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_direct_path_2_1_and_then_any_child_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1].* (direct path 2-1 and then any child) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![75usize, 142usize, 209usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_direct_path_2_1_and_then_any_child_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1].* (direct path 2-1 and then any child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_direct_path_2_1_and_then_any_child_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1].* (direct path 2-1 and then any child) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![75usize, 142usize, 209usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_direct_path_2_1_and_then_any_child_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1].* (direct path 2-1 and then any child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_direct_path_2_1_and_then_any_child_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1].* (direct path 2-1 and then any child) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1].*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![75usize, 142usize, 209usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_all_nodes_at_depth_one_and_then_their_first_list_element_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $.*[0] (select all nodes at depth one and then their first list element) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_all_nodes_at_depth_one_and_then_their_first_list_element_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $.*[0] (select all nodes at depth one and then their first list element) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![16usize, 49usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_all_nodes_at_depth_one_and_then_their_first_list_element_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $.*[0] (select all nodes at depth one and then their first list element) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_all_nodes_at_depth_one_and_then_their_first_list_element_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $.*[0] (select all nodes at depth one and then their first list element) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![16usize, 49usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_all_nodes_at_depth_one_and_then_their_first_list_element_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $.*[0] (select all nodes at depth one and then their first list element) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_all_nodes_at_depth_one_and_then_their_first_list_element_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $.*[0] (select all nodes at depth one and then their first list element) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![16usize, 49usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_all_non_root_nodes_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $..* (select all non-root nodes) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 16u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_all_non_root_nodes_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $..* (select all non-root nodes) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            6usize, 16usize, 17usize, 31usize, 39usize, 49usize, 61usize, 75usize, 93usize, 113usize, 142usize,
            160usize, 180usize, 209usize, 227usize, 247usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_all_non_root_nodes_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $..* (select all non-root nodes) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 16u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_all_non_root_nodes_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $..* (select all non-root nodes) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            6usize, 16usize, 17usize, 31usize, 39usize, 49usize, 61usize, 75usize, 93usize, 113usize, 142usize,
            160usize, 180usize, 209usize, 227usize, 247usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_all_non_root_nodes_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $..* (select all non-root nodes) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 16u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_all_non_root_nodes_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $..* (select all non-root nodes) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            6usize, 16usize, 17usize, 31usize, 39usize, 49usize, 61usize, 75usize, 93usize, 113usize, 142usize,
            160usize, 180usize, 209usize, 227usize, 247usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_and_then_all_its_children_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[0].* (select the first element and then all its children) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_and_then_all_its_children_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[0].* (select the first element and then all its children) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![16usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_and_then_all_its_children_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[0].* (select the first element and then all its children) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_and_then_all_its_children_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[0].* (select the first element and then all its children) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![16usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_and_then_all_its_children_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[0].* (select the first element and then all its children) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_and_then_all_its_children_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[0].* (select the first element and then all its children) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0].*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![16usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_of_every_nested_list_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $..*[0] (select the first element of every nested list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_of_every_nested_list_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $..*[0] (select the first element of every nested list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 17usize, 49usize, 75usize, 93usize, 160usize, 227usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_of_every_nested_list_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $..*[0] (select the first element of every nested list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_of_every_nested_list_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $..*[0] (select the first element of every nested list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 17usize, 49usize, 75usize, 93usize, 160usize, 227usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_of_every_nested_list_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $..*[0] (select the first element of every nested list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 7u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_of_every_nested_list_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $..*[0] (select the first element of every nested list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![16usize, 17usize, 49usize, 75usize, 93usize, 160usize, 227usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_of_the_long_list_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][0] (select the first element of the long list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_of_the_long_list_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][0] (select the first element of the long list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![49usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_of_the_long_list_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][0] (select the first element of the long list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_of_the_long_list_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][0] (select the first element of the long list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![49usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_of_the_long_list_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][0] (select the first element of the long list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_of_the_long_list_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][0] (select the first element of the long list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![49usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[0] (select the first element) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[0] (select the first element) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[0] (select the first element) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[0] (select the first element) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[0] (select the first element) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_first_element_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[0] (select the first element) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_long_list_and_then_its_every_subdocument_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2]..* (select the long list and then its every subdocument) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_long_list_and_then_its_every_subdocument_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2]..* (select the long list and then its every subdocument) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![49usize, 61usize, 75usize, 93usize, 113usize, 142usize, 160usize, 180usize, 209usize, 227usize, 247usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_long_list_and_then_its_every_subdocument_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2]..* (select the long list and then its every subdocument) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_long_list_and_then_its_every_subdocument_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2]..* (select the long list and then its every subdocument) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![49usize, 61usize, 75usize, 93usize, 113usize, 142usize, 160usize, 180usize, 209usize, 227usize, 247usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_long_list_and_then_its_every_subdocument_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2]..* (select the long list and then its every subdocument) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 11u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_long_list_and_then_its_every_subdocument_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2]..* (select the long list and then its every subdocument) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2]..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![49usize, 61usize, 75usize, 93usize, 113usize, 142usize, 160usize, 180usize, 209usize, 227usize, 247usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_long_list_then_go_two_levels_down_and_select_the_second_element_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2].*.*[1] (select the long list, then go two levels down and select the second element) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*.*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_long_list_then_go_two_levels_down_and_select_the_second_element_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2].*.*[1] (select the long list, then go two levels down and select the second element) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*.*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![113usize, 180usize, 247usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_long_list_then_go_two_levels_down_and_select_the_second_element_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2].*.*[1] (select the long list, then go two levels down and select the second element) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*.*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_long_list_then_go_two_levels_down_and_select_the_second_element_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2].*.*[1] (select the long list, then go two levels down and select the second element) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*.*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![113usize, 180usize, 247usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_long_list_then_go_two_levels_down_and_select_the_second_element_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2].*.*[1] (select the long list, then go two levels down and select the second element) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*.*[1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_long_list_then_go_two_levels_down_and_select_the_second_element_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2].*.*[1] (select the long list, then go two levels down and select the second element) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*.*[1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![113usize, 180usize, 247usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_long_list_then_in_each_sublist_select_the_second_element_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2].*[1] (select the long list, then in each sublist select the second element) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_long_list_then_in_each_sublist_select_the_second_element_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2].*[1] (select the long list, then in each sublist select the second element) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![142usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_long_list_then_in_each_sublist_select_the_second_element_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2].*[1] (select the long list, then in each sublist select the second element) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_long_list_then_in_each_sublist_select_the_second_element_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2].*[1] (select the long list, then in each sublist select the second element) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![142usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_long_list_then_in_each_sublist_select_the_second_element_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2].*[1] (select the long list, then in each sublist select the second element) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_long_list_then_in_each_sublist_select_the_second_element_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2].*[1] (select the long list, then in each sublist select the second element) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2].*[1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![142usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_path_2_1_then_in_any_subtree_select_the_second_list_element_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1]..*[1] (select the path 2-1, then in any subtree select the second list element) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_path_2_1_then_in_any_subtree_select_the_second_list_element_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1]..*[1] (select the path 2-1, then in any subtree select the second list element) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![113usize, 180usize, 247usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_path_2_1_then_in_any_subtree_select_the_second_list_element_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1]..*[1] (select the path 2-1, then in any subtree select the second list element) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_path_2_1_then_in_any_subtree_select_the_second_list_element_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1]..*[1] (select the path 2-1, then in any subtree select the second list element) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![113usize, 180usize, 247usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_path_2_1_then_in_any_subtree_select_the_second_list_element_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1]..*[1] (select the path 2-1, then in any subtree select the second list element) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_path_2_1_then_in_any_subtree_select_the_second_list_element_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1]..*[1] (select the path 2-1, then in any subtree select the second list element) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]..*[1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![113usize, 180usize, 247usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_second_element_of_the_long_list_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1] (select the second element of the long list) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_second_element_of_the_long_list_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1] (select the second element of the long list) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![61usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_second_element_of_the_long_list_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1] (select the second element of the long list) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_second_element_of_the_long_list_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1] (select the second element of the long list) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![61usize,], "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_second_element_of_the_long_list_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1] (select the second element of the long list) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn only_lists_and_integers_nested_in_each_other_with_query_select_the_second_element_of_the_long_list_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document lists.toml running the query $[2][1] (select the second element of the long list) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$[2][1]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/lists.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![61usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_descendant_search_for_number_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_descendant_search_for_number_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![169usize, 211usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_descendant_search_for_number_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_descendant_search_for_number_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![169usize, 211usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_descendant_search_for_number_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_descendant_search_for_number_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![169usize, 211usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_exact_path_with_name_and_index_selectors_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_exact_path_with_name_and_index_selectors_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_exact_path_with_name_and_index_selectors_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_exact_path_with_name_and_index_selectors_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_exact_path_with_name_and_index_selectors_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_exact_path_with_name_and_index_selectors_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![151usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_first_number_directly_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example.toml running the query $.phoneNumbers[0] (select first number directly) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_first_number_directly_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example.toml running the query $.phoneNumbers[0] (select first number directly) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![143usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_first_number_directly_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example.toml running the query $.phoneNumbers[0] (select first number directly) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_first_number_directly_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example.toml running the query $.phoneNumbers[0] (select first number directly) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![143usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_first_number_directly_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example.toml running the query $.phoneNumbers[0] (select first number directly) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_compressed_with_query_select_first_number_directly_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example.toml running the query $.phoneNumbers[0] (select first number directly) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![143usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_descendant_search_for_number_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_descendant_search_for_number_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![271usize, 359usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_descendant_search_for_number_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_descendant_search_for_number_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![271usize, 359usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_descendant_search_for_number_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/jsonpath_com_example.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_descendant_search_for_number_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/jsonpath_com_example.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![271usize, 359usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_exact_path_with_name_and_index_selectors_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_exact_path_with_name_and_index_selectors_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![239usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_exact_path_with_name_and_index_selectors_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_exact_path_with_name_and_index_selectors_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![239usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_exact_path_with_name_and_index_selectors_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/jsonpath_com_example.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_exact_path_with_name_and_index_selectors_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/jsonpath_com_example.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![239usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_first_number_directly_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0] (select first number directly) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_first_number_directly_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0] (select first number directly) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/jsonpath_com_example.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![217usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_first_number_directly_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0] (select first number directly) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_first_number_directly_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0] (select first number directly) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/jsonpath_com_example.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![217usize,], "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_first_number_directly_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0] (select first number directly) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/jsonpath_com_example.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_with_query_select_first_number_directly_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0] (select first number directly) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/jsonpath_com_example.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![217usize,], "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_at_depth_at_least_2_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_at_depth_at_least_2_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize, 15usize,], "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_at_depth_at_least_2_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_at_depth_at_least_2_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize, 15usize,], "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_at_depth_at_least_2_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_at_depth_at_least_2_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize, 15usize,], "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_at_depth_exactly_1_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $.* (select all nodes at depth exactly 1) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_at_depth_exactly_1_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $.* (select all nodes at depth exactly 1) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![5usize, 14usize, 23usize, 30usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_at_depth_exactly_1_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $.* (select all nodes at depth exactly 1) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_at_depth_exactly_1_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $.* (select all nodes at depth exactly 1) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![5usize, 14usize, 23usize, 30usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_at_depth_exactly_1_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $.* (select all nodes at depth exactly 1) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_at_depth_exactly_1_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $.* (select all nodes at depth exactly 1) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![5usize, 14usize, 23usize, 30usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_at_depth_exactly_2_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $.*.* (select all nodes at depth exactly 2) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_at_depth_exactly_2_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $.*.* (select all nodes at depth exactly 2) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize, 15usize,], "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_at_depth_exactly_2_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $.*.* (select all nodes at depth exactly 2) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_at_depth_exactly_2_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $.*.* (select all nodes at depth exactly 2) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize, 15usize,], "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_at_depth_exactly_2_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $.*.* (select all nodes at depth exactly 2) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_at_depth_exactly_2_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $.*.* (select all nodes at depth exactly 2) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize, 15usize,], "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $..* (select all nodes) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $..* (select all nodes) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![5usize, 6usize, 14usize, 15usize, 23usize, 30usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $..* (select all nodes) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $..* (select all nodes) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![5usize, 6usize, 14usize, 15usize, 23usize, 30usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $..* (select all nodes) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_compressed_with_query_select_all_nodes_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/singletons_and_empties.toml running the query $..* (select all nodes) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![5usize, 6usize, 14usize, 15usize, 23usize, 30usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_at_depth_at_least_2_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_at_depth_at_least_2_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_at_depth_at_least_2_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_at_depth_at_least_2_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_at_depth_at_least_2_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_at_depth_at_least_2_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..*.* (select all nodes at depth at least 2) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_at_depth_exactly_1_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.* (select all nodes at depth exactly 1) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_at_depth_exactly_1_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.* (select all nodes at depth exactly 1) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![11usize, 40usize, 69usize, 82usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_at_depth_exactly_1_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.* (select all nodes at depth exactly 1) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_at_depth_exactly_1_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.* (select all nodes at depth exactly 1) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![11usize, 40usize, 69usize, 82usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_at_depth_exactly_1_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.* (select all nodes at depth exactly 1) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_at_depth_exactly_1_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.* (select all nodes at depth exactly 1) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![11usize, 40usize, 69usize, 82usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_at_depth_exactly_2_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*.* (select all nodes at depth exactly 2) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_at_depth_exactly_2_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*.* (select all nodes at depth exactly 2) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_at_depth_exactly_2_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*.* (select all nodes at depth exactly 2) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_at_depth_exactly_2_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*.* (select all nodes at depth exactly 2) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_at_depth_exactly_2_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*.* (select all nodes at depth exactly 2) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_at_depth_exactly_2_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $.*.* (select all nodes at depth exactly 2) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.*.*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![21usize, 50usize,], "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..* (select all nodes) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..* (select all nodes) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![11usize, 21usize, 40usize, 50usize, 69usize, 82usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..* (select all nodes) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..* (select all nodes) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![11usize, 21usize, 40usize, 50usize, 69usize, 82usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..* (select all nodes) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 6u64, "result != expected");
    Ok(())
}
#[test]
fn single_element_lists_empty_lists_and_empty_objects_with_query_select_all_nodes_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document singletons_and_empties.toml running the query $..* (select all nodes) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..*")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/singletons_and_empties.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![11usize, 21usize, 40usize, 50usize, 69usize, 82usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_compressed_with_query_descendant_search_for_person_phone_number_number_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example_extended.toml running the query $..person..phoneNumber..number (descendant search for person phoneNumber number) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let json_file =
        fs::File::open("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example_extended.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_compressed_with_query_descendant_search_for_person_phone_number_number_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example_extended.toml running the query $..person..phoneNumber..number (descendant search for person phoneNumber number) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let json_file =
        fs::File::open("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example_extended.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![178usize, 220usize, 426usize, 468usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_compressed_with_query_descendant_search_for_person_phone_number_number_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example_extended.toml running the query $..person..phoneNumber..number (descendant search for person phoneNumber number) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let json_file =
        fs::File::open("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example_extended.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_compressed_with_query_descendant_search_for_person_phone_number_number_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example_extended.toml running the query $..person..phoneNumber..number (descendant search for person phoneNumber number) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let json_file =
        fs::File::open("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example_extended.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![178usize, 220usize, 426usize, 468usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_compressed_with_query_descendant_search_for_person_phone_number_number_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example_extended.toml running the query $..person..phoneNumber..number (descendant search for person phoneNumber number) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let raw_json =
        fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example_extended.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_compressed_with_query_descendant_search_for_person_phone_number_number_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example_extended.toml running the query $..person..phoneNumber..number (descendant search for person phoneNumber number) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let raw_json =
        fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example_extended.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![178usize, 220usize, 426usize, 468usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_compressed_with_query_descendant_search_for_person_then_any_node_then_type_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example_extended.toml running the query $..person..*..type (descendant search for person, then any node, then type) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let json_file =
        fs::File::open("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example_extended.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_compressed_with_query_descendant_search_for_person_then_any_node_then_type_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example_extended.toml running the query $..person..*..type (descendant search for person, then any node, then type) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let json_file =
        fs::File::open("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example_extended.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![160usize, 204usize, 408usize, 452usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_compressed_with_query_descendant_search_for_person_then_any_node_then_type_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example_extended.toml running the query $..person..*..type (descendant search for person, then any node, then type) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let json_file =
        fs::File::open("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example_extended.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_compressed_with_query_descendant_search_for_person_then_any_node_then_type_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example_extended.toml running the query $..person..*..type (descendant search for person, then any node, then type) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let json_file =
        fs::File::open("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example_extended.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![160usize, 204usize, 408usize, 452usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_compressed_with_query_descendant_search_for_person_then_any_node_then_type_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example_extended.toml running the query $..person..*..type (descendant search for person, then any node, then type) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let raw_json =
        fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example_extended.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_compressed_with_query_descendant_search_for_person_then_any_node_then_type_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/jsonpath_com_example_extended.toml running the query $..person..*..type (descendant search for person, then any node, then type) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let raw_json =
        fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/jsonpath_com_example_extended.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![160usize, 204usize, 408usize, 452usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_with_query_descendant_search_for_person_phone_number_number_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example_extended.toml running the query $..person..phoneNumber..number (descendant search for person phoneNumber number) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/jsonpath_com_example_extended.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_with_query_descendant_search_for_person_phone_number_number_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example_extended.toml running the query $..person..phoneNumber..number (descendant search for person phoneNumber number) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/jsonpath_com_example_extended.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![334usize, 438usize, 936usize, 1072usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_with_query_descendant_search_for_person_phone_number_number_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example_extended.toml running the query $..person..phoneNumber..number (descendant search for person phoneNumber number) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/jsonpath_com_example_extended.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_with_query_descendant_search_for_person_phone_number_number_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example_extended.toml running the query $..person..phoneNumber..number (descendant search for person phoneNumber number) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/jsonpath_com_example_extended.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![334usize, 438usize, 936usize, 1072usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_with_query_descendant_search_for_person_phone_number_number_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example_extended.toml running the query $..person..phoneNumber..number (descendant search for person phoneNumber number) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/jsonpath_com_example_extended.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_with_query_descendant_search_for_person_phone_number_number_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example_extended.toml running the query $..person..phoneNumber..number (descendant search for person phoneNumber number) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..phoneNumber..number")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/jsonpath_com_example_extended.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![334usize, 438usize, 936usize, 1072usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_with_query_descendant_search_for_person_then_any_node_then_type_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example_extended.toml running the query $..person..*..type (descendant search for person, then any node, then type) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/jsonpath_com_example_extended.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_with_query_descendant_search_for_person_then_any_node_then_type_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example_extended.toml running the query $..person..*..type (descendant search for person, then any node, then type) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/jsonpath_com_example_extended.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![298usize, 404usize, 892usize, 1030usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_with_query_descendant_search_for_person_then_any_node_then_type_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example_extended.toml running the query $..person..*..type (descendant search for person, then any node, then type) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/jsonpath_com_example_extended.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_with_query_descendant_search_for_person_then_any_node_then_type_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example_extended.toml running the query $..person..*..type (descendant search for person, then any node, then type) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/jsonpath_com_example_extended.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![298usize, 404usize, 892usize, 1030usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_with_query_descendant_search_for_person_then_any_node_then_type_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example_extended.toml running the query $..person..*..type (descendant search for person, then any node, then type) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/jsonpath_com_example_extended.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 4u64, "result != expected");
    Ok(())
}
#[test]
fn the_example_on_jsonpath_com_extended_with_another_nested_person_object_with_query_descendant_search_for_person_then_any_node_then_type_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example_extended.toml running the query $..person..*..type (descendant search for person, then any node, then type) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..person..*..type")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/jsonpath_com_example_extended.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![298usize, 404usize, 892usize, 1030usize,],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_then_child_url_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..user..entities.url (descendant user entities, then child url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 18u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_then_child_url_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..user..entities.url (descendant user entities, then child url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            3488usize,
            9836usize,
            12718usize,
            52574usize,
            64603usize,
            77997usize,
            119307usize,
            121918usize,
            201073usize,
            212698usize,
            215343usize,
            241826usize,
            288269usize,
            310030usize,
            312972usize,
            445431usize,
            454460usize,
            464576usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_then_child_url_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..user..entities.url (descendant user entities, then child url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 18u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_then_child_url_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..user..entities.url (descendant user entities, then child url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            3488usize,
            9836usize,
            12718usize,
            52574usize,
            64603usize,
            77997usize,
            119307usize,
            121918usize,
            201073usize,
            212698usize,
            215343usize,
            241826usize,
            288269usize,
            310030usize,
            312972usize,
            445431usize,
            454460usize,
            464576usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_then_child_url_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..user..entities.url (descendant user entities, then child url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 18u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_then_child_url_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..user..entities.url (descendant user entities, then child url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            3488usize,
            9836usize,
            12718usize,
            52574usize,
            64603usize,
            77997usize,
            119307usize,
            121918usize,
            201073usize,
            212698usize,
            215343usize,
            241826usize,
            288269usize,
            310030usize,
            312972usize,
            445431usize,
            454460usize,
            464576usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_url_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 44u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_url_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            3488usize,
            3504usize,
            5803usize,
            5955usize,
            9836usize,
            9852usize,
            12718usize,
            12734usize,
            12913usize,
            52574usize,
            52590usize,
            64603usize,
            64619usize,
            77997usize,
            78013usize,
            78165usize,
            119307usize,
            119323usize,
            121918usize,
            121934usize,
            122097usize,
            201073usize,
            201089usize,
            212698usize,
            212714usize,
            212878usize,
            215343usize,
            215359usize,
            241826usize,
            241842usize,
            274278usize,
            288269usize,
            288285usize,
            310030usize,
            310046usize,
            312972usize,
            312988usize,
            445431usize,
            445447usize,
            454460usize,
            454476usize,
            464576usize,
            464592usize,
            464769usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_url_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 44u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_url_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            3488usize,
            3504usize,
            5803usize,
            5955usize,
            9836usize,
            9852usize,
            12718usize,
            12734usize,
            12913usize,
            52574usize,
            52590usize,
            64603usize,
            64619usize,
            77997usize,
            78013usize,
            78165usize,
            119307usize,
            119323usize,
            121918usize,
            121934usize,
            122097usize,
            201073usize,
            201089usize,
            212698usize,
            212714usize,
            212878usize,
            215343usize,
            215359usize,
            241826usize,
            241842usize,
            274278usize,
            288269usize,
            288285usize,
            310030usize,
            310046usize,
            312972usize,
            312988usize,
            445431usize,
            445447usize,
            454460usize,
            454476usize,
            464576usize,
            464592usize,
            464769usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_url_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 44u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_descendant_user_entities_url_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            3488usize,
            3504usize,
            5803usize,
            5955usize,
            9836usize,
            9852usize,
            12718usize,
            12734usize,
            12913usize,
            52574usize,
            52590usize,
            64603usize,
            64619usize,
            77997usize,
            78013usize,
            78165usize,
            119307usize,
            119323usize,
            121918usize,
            121934usize,
            122097usize,
            201073usize,
            201089usize,
            212698usize,
            212714usize,
            212878usize,
            215343usize,
            215359usize,
            241826usize,
            241842usize,
            274278usize,
            288269usize,
            288285usize,
            310030usize,
            310046usize,
            312972usize,
            312988usize,
            445431usize,
            445447usize,
            454460usize,
            454476usize,
            464576usize,
            464592usize,
            464769usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_count_by_descendant_should_use_memchr_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..count (select count by descendant (should use memchr)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_count_by_descendant_should_use_memchr_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..count (select count by descendant (should use memchr)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![466869usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_count_by_descendant_should_use_memchr_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..count (select count by descendant (should use memchr)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_count_by_descendant_should_use_memchr_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..count (select count by descendant (should use memchr)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![466869usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_count_by_descendant_should_use_memchr_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..count (select count by descendant (should use memchr)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_count_by_descendant_should_use_memchr_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..count (select count by descendant (should use memchr)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![466869usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_and_count_by_descendant_should_use_memchr_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..search_metadata..count (select metadata and count by descendant (should use memchr)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_and_count_by_descendant_should_use_memchr_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..search_metadata..count (select metadata and count by descendant (should use memchr)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![466869usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_and_count_by_descendant_should_use_memchr_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..search_metadata..count (select metadata and count by descendant (should use memchr)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_and_count_by_descendant_should_use_memchr_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..search_metadata..count (select metadata and count by descendant (should use memchr)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![466869usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_and_count_by_descendant_should_use_memchr_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..search_metadata..count (select metadata and count by descendant (should use memchr)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_and_count_by_descendant_should_use_memchr_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..search_metadata..count (select metadata and count by descendant (should use memchr)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![466869usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_count_by_descendant_should_use_memchr_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..search_metadata.count (select metadata count by descendant (should use memchr)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_count_by_descendant_should_use_memchr_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..search_metadata.count (select metadata count by descendant (should use memchr)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![466869usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_count_by_descendant_should_use_memchr_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..search_metadata.count (select metadata count by descendant (should use memchr)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_count_by_descendant_should_use_memchr_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..search_metadata.count (select metadata count by descendant (should use memchr)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![466869usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_count_by_descendant_should_use_memchr_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..search_metadata.count (select metadata count by descendant (should use memchr)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_count_by_descendant_should_use_memchr_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $..search_metadata.count (select metadata count by descendant (should use memchr)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![466869usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_count_directly_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $.search_metadata.count (select metadata count directly) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_count_directly_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $.search_metadata.count (select metadata count directly) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![466869usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_count_directly_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $.search_metadata.count (select metadata count directly) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_count_directly_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $.search_metadata.count (select metadata count directly) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![466869usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_count_directly_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $.search_metadata.count (select metadata count directly) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_count_directly_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $.search_metadata.count (select metadata count directly) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![466869usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_directly_and_then_count_by_descendant_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $.search_metadata..count (select metadata directly, and then count by descendant) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_directly_and_then_count_by_descendant_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $.search_metadata..count (select metadata directly, and then count by descendant) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![466869usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_directly_and_then_count_by_descendant_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $.search_metadata..count (select metadata directly, and then count by descendant) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_directly_and_then_count_by_descendant_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $.search_metadata..count (select metadata directly, and then count by descendant) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![466869usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_directly_and_then_count_by_descendant_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $.search_metadata..count (select metadata directly, and then count by descendant) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_compressed_with_query_select_metadata_directly_and_then_count_by_descendant_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/twitter.toml running the query $.search_metadata..count (select metadata directly, and then count by descendant) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/compressed/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![466869usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_then_child_url_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities.url (descendant user entities, then child url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 18u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_then_child_url_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities.url (descendant user entities, then child url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            5465usize,
            18496usize,
            23338usize,
            89785usize,
            112198usize,
            134220usize,
            201055usize,
            205281usize,
            333130usize,
            352432usize,
            357000usize,
            399785usize,
            475584usize,
            511442usize,
            516538usize,
            728252usize,
            743602usize,
            762797usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_then_child_url_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities.url (descendant user entities, then child url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 18u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_then_child_url_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities.url (descendant user entities, then child url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            5465usize,
            18496usize,
            23338usize,
            89785usize,
            112198usize,
            134220usize,
            201055usize,
            205281usize,
            333130usize,
            352432usize,
            357000usize,
            399785usize,
            475584usize,
            511442usize,
            516538usize,
            728252usize,
            743602usize,
            762797usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_then_child_url_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities.url (descendant user entities, then child url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 18u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_then_child_url_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities.url (descendant user entities, then child url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities.url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            5465usize,
            18496usize,
            23338usize,
            89785usize,
            112198usize,
            134220usize,
            201055usize,
            205281usize,
            333130usize,
            352432usize,
            357000usize,
            399785usize,
            475584usize,
            511442usize,
            516538usize,
            728252usize,
            743602usize,
            762797usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_url_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 44u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_url_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            5465usize,
            5570usize,
            9496usize,
            9985usize,
            18496usize,
            18601usize,
            23338usize,
            23443usize,
            24017usize,
            89785usize,
            89902usize,
            112198usize,
            112315usize,
            134220usize,
            134337usize,
            134936usize,
            201055usize,
            201160usize,
            205281usize,
            205398usize,
            206008usize,
            333130usize,
            333235usize,
            352432usize,
            352537usize,
            353096usize,
            357000usize,
            357117usize,
            399785usize,
            399902usize,
            451854usize,
            475584usize,
            475689usize,
            511442usize,
            511547usize,
            516538usize,
            516643usize,
            728252usize,
            728357usize,
            743602usize,
            743719usize,
            762797usize,
            762902usize,
            763474usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_url_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 44u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_url_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            5465usize,
            5570usize,
            9496usize,
            9985usize,
            18496usize,
            18601usize,
            23338usize,
            23443usize,
            24017usize,
            89785usize,
            89902usize,
            112198usize,
            112315usize,
            134220usize,
            134337usize,
            134936usize,
            201055usize,
            201160usize,
            205281usize,
            205398usize,
            206008usize,
            333130usize,
            333235usize,
            352432usize,
            352537usize,
            353096usize,
            357000usize,
            357117usize,
            399785usize,
            399902usize,
            451854usize,
            475584usize,
            475689usize,
            511442usize,
            511547usize,
            516538usize,
            516643usize,
            728252usize,
            728357usize,
            743602usize,
            743719usize,
            762797usize,
            762902usize,
            763474usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_url_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 44u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_descendant_user_entities_url_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..user..entities..url (descendant user entities url) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..user..entities..url")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(
        result.get(),
        vec![
            5465usize,
            5570usize,
            9496usize,
            9985usize,
            18496usize,
            18601usize,
            23338usize,
            23443usize,
            24017usize,
            89785usize,
            89902usize,
            112198usize,
            112315usize,
            134220usize,
            134337usize,
            134936usize,
            201055usize,
            201160usize,
            205281usize,
            205398usize,
            206008usize,
            333130usize,
            333235usize,
            352432usize,
            352537usize,
            353096usize,
            357000usize,
            357117usize,
            399785usize,
            399902usize,
            451854usize,
            475584usize,
            475689usize,
            511442usize,
            511547usize,
            516538usize,
            516643usize,
            728252usize,
            728357usize,
            743602usize,
            743719usize,
            762797usize,
            762902usize,
            763474usize,
        ],
        "result != expected"
    );
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_count_by_descendant_should_use_memchr_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..count (select count by descendant (should use memchr)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_count_by_descendant_should_use_memchr_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..count (select count by descendant (should use memchr)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_count_by_descendant_should_use_memchr_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..count (select count by descendant (should use memchr)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_count_by_descendant_should_use_memchr_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..count (select count by descendant (should use memchr)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_count_by_descendant_should_use_memchr_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..count (select count by descendant (should use memchr)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_count_by_descendant_should_use_memchr_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..count (select count by descendant (should use memchr)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..count")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_and_count_by_descendant_should_use_memchr_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata..count (select metadata and count by descendant (should use memchr)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_and_count_by_descendant_should_use_memchr_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata..count (select metadata and count by descendant (should use memchr)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_and_count_by_descendant_should_use_memchr_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata..count (select metadata and count by descendant (should use memchr)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_and_count_by_descendant_should_use_memchr_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata..count (select metadata and count by descendant (should use memchr)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_and_count_by_descendant_should_use_memchr_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata..count (select metadata and count by descendant (should use memchr)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_and_count_by_descendant_should_use_memchr_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata..count (select metadata and count by descendant (should use memchr)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata..count")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_count_by_descendant_should_use_memchr_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata.count (select metadata count by descendant (should use memchr)) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_count_by_descendant_should_use_memchr_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata.count (select metadata count by descendant (should use memchr)) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_count_by_descendant_should_use_memchr_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata.count (select metadata count by descendant (should use memchr)) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_count_by_descendant_should_use_memchr_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata.count (select metadata count by descendant (should use memchr)) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_count_by_descendant_should_use_memchr_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata.count (select metadata count by descendant (should use memchr)) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_count_by_descendant_should_use_memchr_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $..search_metadata.count (select metadata count by descendant (should use memchr)) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..search_metadata.count")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_count_directly_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata.count (select metadata count directly) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_count_directly_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata.count (select metadata count directly) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_count_directly_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata.count (select metadata count directly) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_count_directly_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata.count (select metadata count directly) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_count_directly_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata.count (select metadata count directly) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_count_directly_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata.count (select metadata count directly) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata.count")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_directly_and_then_count_by_descendant_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata..count (select metadata directly, and then count by descendant) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_directly_and_then_count_by_descendant_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata..count (select metadata directly, and then count by descendant) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_directly_and_then_count_by_descendant_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata..count (select metadata directly, and then count by descendant) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_directly_and_then_count_by_descendant_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata..count (select metadata directly, and then count by descendant) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_directly_and_then_count_by_descendant_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata..count (select metadata directly, and then count by descendant) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn twitter_json_from_simdjson_github_example_with_query_select_metadata_directly_and_then_count_by_descendant_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document twitter.toml running the query $.search_metadata..count (select metadata directly, and then count by descendant) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.search_metadata..count")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/large/twitter.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![767233usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_child_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $.a (select a by child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_child_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $.a (select a by child) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_child_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $.a (select a by child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_child_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $.a (select a by child) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_child_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $.a (select a by child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_child_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $.a (select a by child) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_descendant_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $..a (select a by descendant) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_descendant_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $..a (select a by descendant) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_descendant_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $..a (select a by descendant) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_descendant_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $..a (select a by descendant) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_descendant_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $..a (select a by descendant) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_a_by_descendant_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $..a (select a by descendant) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![5usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_each_item_on_the_list_with_wildcard_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_each_item_on_the_list_with_wildcard_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize, 8usize, 10usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_each_item_on_the_list_with_wildcard_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_each_item_on_the_list_with_wildcard_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize, 8usize, 10usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_each_item_on_the_list_with_wildcard_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_each_item_on_the_list_with_wildcard_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![6usize, 8usize, 10usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_root_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $ (select root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_root_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $ (select root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_root_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $ (select root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_root_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $ (select root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_root_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $ (select root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_compressed_with_query_select_root_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document compressed/index_result.toml running the query $ (select root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/compressed/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_child_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_child_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![9usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_child_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_child_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![9usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_child_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_child_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a (select a by child) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![9usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_descendant_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_descendant_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![9usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_descendant_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_descendant_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![9usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_descendant_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_a_by_descendant_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $..a (select a by descendant) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..a")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![9usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_each_item_on_the_list_with_wildcard_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_each_item_on_the_list_with_wildcard_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![10usize, 16usize, 22usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_each_item_on_the_list_with_wildcard_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_each_item_on_the_list_with_wildcard_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![10usize, 16usize, 22usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_each_item_on_the_list_with_wildcard_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 3u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_each_item_on_the_list_with_wildcard_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $.a[*] (select each item on the list with wildcard) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.a[*]")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![10usize, 16usize, 22usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_root_with_buffered_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl BufferedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_root_with_buffered_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl BufferedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = BufferedInput::new(json_file);
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_root_with_mmap_input_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl MmapInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_root_with_mmap_input_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl MmapInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let json_file = fs::File::open("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = unsafe { MmapInput::map_file(&json_file)? };
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_root_with_owned_bytes_and_count_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl OwnedBytes and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1u64, "result != expected");
    Ok(())
}
#[test]
fn whitespace_separators_between_structurals_to_test_correctness_of_index_result_handling_with_query_select_root_with_owned_bytes_and_index_result_using_main_engine(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document index_result.toml running the query $ (select root) with Input impl OwnedBytes and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$")?;
    let raw_json = fs::read_to_string("../rsonpath-lib/tests/documents/json/index_result.json")?;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![0usize,], "result != expected");
    Ok(())
}