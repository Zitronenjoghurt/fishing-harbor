use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EmbedFields<N, V>(HashMap<N, (V, bool)>);

impl Default for EmbedFields<String, String> {
    fn default() -> Self {
        EmbedFields(HashMap::new())
    }
}

impl<N, V> IntoIterator for EmbedFields<N, V> {
    type Item = (N, V, bool);
    type IntoIter = std::iter::Map<
        std::collections::hash_map::IntoIter<N, (V, bool)>,
        fn((N, (V, bool))) -> (N, V, bool),
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .map(|(name, (value, inline))| (name, value, inline))
    }
}
