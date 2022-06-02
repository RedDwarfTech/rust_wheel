use std::collections::HashMap;

pub trait IntoTree: Sized {
    type Output;

    fn get_id(&self) -> i32;
    fn get_parent_id(&self) -> i32;
    fn convert(&self, children: Vec<Self::Output>) -> Self::Output;
}

fn take_all_children<T>(parent_id: i32, sub_menus: &mut HashMap<i32, Vec<&T>>) -> Vec<T::Output>
    where
        T: IntoTree,
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

pub fn convert_to_tree<T>(root_menus: &[T], sub_menus: &[T]) -> Vec<T::Output>
    where
        T: IntoTree,
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



