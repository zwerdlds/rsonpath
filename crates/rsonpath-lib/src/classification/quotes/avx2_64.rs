use super::{
    shared::{mask_64, vector_256},
    *,
};
use crate::{
    block,
    classification::mask::m64,
    debug,
    input::{error::InputErrorConvertible, InputBlock, InputBlockIterator},
    FallibleIterator,
};
use std::marker::PhantomData;

super::shared::quote_classifier!(Avx2QuoteClassifier64, BlockAvx2Classifier, 64, u64);

struct BlockAvx2Classifier {
    internal_classifier: mask_64::BlockClassifier64Bit,
}

impl BlockAvx2Classifier {
    fn new() -> Self {
        Self {
            internal_classifier: mask_64::BlockClassifier64Bit::new(),
        }
    }

    #[inline(always)]
    unsafe fn classify<'a, B: InputBlock<'a, 64>>(&mut self, blocks: &B) -> u64 {
        block!(blocks[..64]);

        let (block1, block2) = blocks.halves();
        let classification1 = vector_256::classify_block(block1);
        let classification2 = vector_256::classify_block(block2);

        let slashes = m64::combine_32(classification1.slashes, classification2.slashes);
        let quotes = m64::combine_32(classification1.quotes, classification2.quotes);

        self.internal_classifier.classify(slashes, quotes)
    }
}

#[cfg(all(test, cfg = "avx_64"))]
mod tests {
    use super::Avx2QuoteClassifier64;
    use crate::{
        input::{Input, OwnedBytes},
        result::empty::EmptyRecorder,
        FallibleIterator,
    };
    use test_case::test_case;

    #[test_case("" => None)]
    #[test_case("abcd" => Some(0))]
    #[test_case(r#""abcd""# => Some(0b01_1111))]
    #[test_case(r#""number": 42, "string": "something" "# => Some(0b0011_1111_1111_0001_1111_1100_0000_0111_1111))]
    #[test_case(r#"abc\"abc\""# => Some(0b00_0000_0000))]
    #[test_case(r#"abc\\"abc\\""# => Some(0b0111_1110_0000))]
    #[test_case(r#"{"aaa":[{},{"b":{"c":[1,2,3]}}],"e":{"a":[[],[1,2,3],"# => Some(0b0_0000_0000_0000_0110_0011_0000_0000_0000_0110_0011_0000_0001_1110))]
    fn single_block(str: &str) -> Option<u64> {
        let owned_str = str.to_owned();
        let input = OwnedBytes::new(&owned_str).unwrap();
        let iter = input.iter_blocks::<_, 64>(&EmptyRecorder);
        let mut classifier = Avx2QuoteClassifier64::new(iter);
        classifier.next().unwrap().map(|x| x.within_quotes_mask)
    }
}
