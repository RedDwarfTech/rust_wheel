use std::collections::HashMap;
use std::hash::Hash;

pub trait IntoTree<E: PartialEq + Eq + Hash>: Sized {
    type Output;

    fn get_id(&self) -> E;
    fn get_parent_id(&self) -> E;
    fn convert(&self, children: Vec<Self::Output>) -> Self::Output;
}

fn take_all_children<T,E>(parent_id: E, sub_menus: &mut HashMap<E, Vec<&T>>) -> Vec<T::Output>
    where
        T: IntoTree<E>,
        E: PartialEq + Eq + Hash,
{
    sub_menus
        .remove(&parent_id)
        .unwrap_or_default()
        .iter()
        .map(|child| {
            let grandchildren = take_all_children(child.get_id(), sub_menus);
            child.convert(grandchildren)
        })
        .collect()
}

pub fn convert_to_tree<T,E>(root_menus: &[T], sub_menus: &[T]) -> Vec<T::Output>
    where
        T: IntoTree<E>,
        E: PartialEq + Eq + Hash,
{
    let mut sub_menus_by_parent = HashMap::new();
    for sub in sub_menus {
        sub_menus_by_parent
            .entry(sub.get_parent_id())
            .or_insert_with(Vec::new)
            .push(sub);
    }

    root_menus
        .iter()
        .map(|root_menu| {
            let children = take_all_children(root_menu.get_id(), &mut sub_menus_by_parent);
            root_menu.convert(children)
        })
        .collect()
}