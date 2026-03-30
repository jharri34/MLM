use std::collections::BTreeSet;

use mlm_db::Category;

use super::libation_cats::CategoryMapping;

pub fn map_storytel_categories(
    category: Option<&str>,
    genres: &[String],
    kids_book: bool,
) -> CategoryMapping {
    let mut categories = BTreeSet::new();
    let mut freeform_tags = BTreeSet::new();

    if kids_book {
        categories.insert(Category::Children);
    }

    for value in category
        .into_iter()
        .chain(genres.iter().map(String::as_str))
    {
        match value.trim() {
            "" | "Audiobook" => {}
            "Crime" => {
                categories.insert(Category::Crime);
            }
            "Fiction" => {
                freeform_tags.insert("Literature & Fiction".to_string());
            }
            "Teens & Young Adult" => {
                categories.insert(Category::YoungAdult);
            }
            other => {
                freeform_tags.insert(other.to_string());
            }
        }
    }

    CategoryMapping {
        categories: categories.into_iter().collect(),
        freeform_tags: freeform_tags.into_iter().collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_storytel_categories() {
        let mapped = map_storytel_categories(
            Some("Teens & Young Adult"),
            &["Audiobook".to_string(), "Teens & Young Adult".to_string()],
            false,
        );

        assert_eq!(mapped.categories, vec![Category::YoungAdult]);
        assert!(mapped.freeform_tags.is_empty());
    }

    #[test]
    fn test_map_storytel_fiction_to_existing_tag() {
        let mapped = map_storytel_categories(
            Some("Fiction"),
            &["Audiobook".to_string(), "Fiction".to_string()],
            false,
        );

        assert!(mapped.categories.is_empty());
        assert_eq!(mapped.freeform_tags, vec!["Literature & Fiction"]);
    }

    #[test]
    fn test_map_storytel_kids_book() {
        let mapped = map_storytel_categories(None, &[], true);

        assert_eq!(mapped.categories, vec![Category::Children]);
        assert!(mapped.freeform_tags.is_empty());
    }
}
