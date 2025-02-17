use std::collections::HashMap;

use asset::{scene, TextureID};
use cgmath::{Quaternion, Rotation, Vector2, Vector3, Zero};
use libsm64::LevelTriangle;

use crate::{
    data::PropInfoContainer,
    graphics::{Canvas, Graphics},
    math::{Ray, Snap},
};

use super::{
    camera::Camera,
    common::Axis,
    elements::{
        self, ElementKind, FaceLocator, Movable, PointLocator, Prop, RaycastHit, RaycastInput,
        RaycastInput2, Solid,
    },
};

#[derive(Default)]
pub struct Scene {
    solids: HashMap<usize, Solid>,
    props: HashMap<usize, Prop>,
    next_elem_id: usize,
    undo_stack: UndoStack,
    redo_stack: Vec<Action>,
}

impl Scene {
    pub fn act(&mut self, ctx: Context, action: Action) {
        if let Some(reaction) = self.execute(ctx, action) {
            self.undo_stack.push(reaction);
            self.redo_stack.clear();
        }
    }

    pub fn undo(&mut self, ctx: Context) {
        if let Some(action) = self.undo_stack.pop() {
            if let Some(reaction) = self.execute(ctx, action) {
                self.redo_stack.push(reaction);
            }
        }
    }

    pub fn redo(&mut self, ctx: Context) {
        if let Some(action) = self.redo_stack.pop() {
            if let Some(reaction) = self.execute(ctx, action) {
                self.undo_stack.push(reaction);
            }
        }
    }

    pub fn raycast(
        &self,
        screen_pos: Vector2<f32>,
        camera: &Camera,
        prop_infos: &PropInfoContainer,
    ) -> RaycastHit {
        elements::raycast(RaycastInput {
            solids: &self.solids,
            props: &self.props,
            camera,
            prop_infos,
            screen_pos,
        })
    }

    pub fn raycast_simple(
        &self,
        camera: &Camera,
        prop_infos: &PropInfoContainer,
        ray: Ray,
    ) -> RaycastHit {
        elements::raycast2(RaycastInput2 {
            solids: &self.solids,
            props: &self.props,
            camera,
            prop_infos,
            ray,
            screen_pos: Vector2::zero(),
        })
    }

    pub fn take_solids(&mut self, mask: ElementKind) -> Vec<(usize, Solid)> {
        #[allow(clippy::needless_collect)]
        let ids = self
            .solids
            .iter()
            .filter_map(|(id, solid)| {
                match mask {
                    ElementKind::Solid => solid.selected(),
                    ElementKind::Face => solid.any_face_selected(),
                    ElementKind::Point => solid.any_point_selected(),
                    ElementKind::Prop => false,
                }
                .then(|| *id)
            })
            .collect::<Vec<_>>();

        ids.into_iter()
            .map(|id| (id, self.solids.remove(&id).unwrap()))
            .collect()
    }

    pub fn clone_solids(&mut self, ctx: Context) -> Vec<(usize, Solid)> {
        self.solids
            .iter()
            .filter_map(|(id, solid)| solid.selected().then(|| (*id, solid.clone(ctx.graphics))))
            .collect()
    }

    pub fn insert_solids_with_move(
        &mut self,
        solids: Vec<(usize, Solid)>,
        delta: Vector3<i32>,
        mask: ElementKind,
    ) {
        for (id, solid) in solids {
            self.solids.insert(id, solid);
        }

        self.undo_stack.push(Action::Move {
            kind: mask,
            delta: -delta,
        });
        self.redo_stack.clear();
    }

    pub fn insert_solids(&mut self, solids: Vec<(usize, Solid)>) {
        for (id, solid) in solids {
            self.solids.insert(id, solid);
        }
    }

    pub fn insert_solids_with_remove(&mut self, solids: Vec<(usize, Solid)>) {
        let mut ids = Vec::new();
        for (_, solid) in solids {
            let id = self.next_elem_id;
            self.next_elem_id += 1;
            self.solids.insert(id, solid);
            ids.push(id);
        }
        self.undo_stack.push(Action::RemoveSolids(ids));
        self.redo_stack.clear();
    }

    pub fn take_props(&mut self) -> Vec<(usize, Prop)> {
        #[allow(clippy::needless_collect)]
        let ids = self
            .props
            .iter()
            .filter_map(|(id, prop)| prop.selected().then(|| *id))
            .collect::<Vec<_>>();

        ids.into_iter()
            .map(|id| (id, self.props.remove(&id).unwrap()))
            .collect()
    }

    pub fn clone_props(&mut self, ctx: Context) -> Vec<(usize, Prop)> {
        self.props
            .iter()
            .filter_map(|(id, prop)| prop.selected().then(|| (*id, prop.clone(ctx.graphics))))
            .collect()
    }

    pub fn insert_props_with_move(&mut self, props: Vec<(usize, Prop)>, delta: Vector3<i32>) {
        for (id, prop) in props {
            self.props.insert(id, prop);
        }

        if delta != Vector3::zero() {
            self.undo_stack.push(Action::Move {
                kind: ElementKind::Prop,
                delta: -delta,
            });
            self.redo_stack.clear();
        }
    }

    pub fn insert_props_with_rotate(&mut self, props: Vec<(usize, Prop)>, delta: Quaternion<f32>) {
        for (id, prop) in props {
            self.props.insert(id, prop);
        }

        self.undo_stack.push(Action::RotateProps(delta.invert()));
        self.redo_stack.clear();
    }

    pub fn insert_props(&mut self, props: Vec<(usize, Prop)>) {
        for (id, prop) in props {
            self.props.insert(id, prop);
        }
    }

    pub fn insert_props_with_remove(&mut self, props: Vec<(usize, Prop)>) {
        let mut ids = Vec::new();

        for (_, prop) in props {
            let id = self.next_elem_id;
            self.next_elem_id += 1;
            self.props.insert(id, prop);
            ids.push(id);
        }

        self.undo_stack.push(Action::RemoveProps(ids));
        self.redo_stack.clear();
    }

    pub fn calc_center(&self, mask: ElementKind) -> Option<Vector3<f32>> {
        match mask {
            ElementKind::Solid => {
                let mut center = Vector3::zero();
                let mut n = 0.0;
                for solid in self.solids.values().filter(|solid| solid.selected()) {
                    center += solid.center(mask);
                    n += 1.0;
                }

                (n > 0.5).then(|| center / n)
            }
            ElementKind::Face => {
                let mut center = Vector3::zero();
                let mut n = 0.0;

                for solid in self.solids.values() {
                    if solid.any_face_selected() {
                        center += solid.center(mask);
                        n += 1.0;
                    }
                }

                (n > 0.5).then(|| center / n)
            }
            ElementKind::Point => {
                let mut center = Vector3::zero();
                let mut n = 0.0;
                for solid in self.solids.values() {
                    if solid.any_point_selected() {
                        center += solid.center(mask);
                        n += 1.0;
                    }
                }

                (n > 0.5).then(|| center / n)
            }
            ElementKind::Prop => {
                let mut center = Vector3::zero();
                let mut n = 0.0;
                for prop in self.props.values().filter(|prop| prop.selected()) {
                    center += prop.center(mask);
                    n += 1.0;
                }

                (n > 0.5).then(|| center / n)
            }
        }
    }

    pub fn any_solids_selected(&self) -> bool {
        self.solids.values().any(|solid| solid.selected())
    }

    pub fn selected_solid_ids(&self) -> Vec<usize> {
        self.solids
            .iter()
            .filter_map(|(id, solid)| solid.selected().then(|| *id))
            .collect()
    }

    pub fn hollow_of(&self, gfx: &Graphics, id: usize, grid: i32) -> Vec<Solid> {
        let solid = self.solids.get(&id).unwrap();
        solid.make_hollow(gfx, grid).into()
    }

    pub fn level_geometry(&self, info: &PropInfoContainer) -> Vec<LevelTriangle> {
        let mut triangles = Vec::with_capacity(self.solids.len() * 12);

        for solid in self.solids.values() {
            triangles.extend(solid.level_triangles().into_iter().flatten());
        }

        for prop in self.props.values() {
            triangles.extend(prop.level_triangles(info).into_iter().flatten());
        }

        triangles
    }

    pub fn render(&self, canvas: &mut Canvas, mask: ElementKind) {
        for solid in self.solids.values() {
            solid.render(canvas, mask);
        }

        for prop in self.props.values() {
            prop.render(canvas, ElementKind::Prop);
        }
    }

    pub fn save(&self) -> scene::World {
        let solids = self.solids.values().map(|solid| solid.save()).collect();
        let props = self.props.values().map(|prop| prop.save()).collect();
        scene::World { solids, props }
    }

    pub fn load(&mut self, ctx: Context, world: &scene::World) {
        self.next_elem_id = 0;
        self.solids = world
            .solids
            .iter()
            .map(|solid| {
                let index = self.next_elem_id;
                self.next_elem_id += 1;
                let solid = Solid::load(ctx.graphics, solid);
                (index, solid)
            })
            .collect();

        self.props = world
            .props
            .iter()
            .map(|prop| {
                let index = self.next_elem_id;
                self.next_elem_id += 1;
                let prop = Prop::load(ctx.graphics, prop);
                (index, prop)
            })
            .collect()
    }

    fn execute(&mut self, ctx: Context, action: Action) -> Option<Action> {
        match action {
            Action::NewSolids(solids) => {
                let ids = solids
                    .into_iter()
                    .map(|solid| {
                        let id = self.next_elem_id;
                        self.next_elem_id += 1;
                        self.solids.insert(id, solid);
                        id
                    })
                    .collect::<Vec<_>>();

                (!ids.is_empty()).then(|| Action::RemoveSolids(ids))
            }

            Action::NewProps(props) => {
                let ids = props
                    .into_iter()
                    .map(|prop| {
                        let id = self.next_elem_id;
                        self.next_elem_id += 1;
                        self.props.insert(id, prop);
                        id
                    })
                    .collect::<Vec<_>>();

                (!ids.is_empty()).then(|| Action::RemoveProps(ids))
            }

            Action::AddSolids(solids) => {
                let mut ids = Vec::new();

                for (id, solid) in solids {
                    self.solids.insert(id, solid);
                    ids.push(id);
                }

                (!ids.is_empty()).then(|| Action::RemoveSolids(ids))
            }

            Action::AddProps(props) => {
                let mut ids = Vec::new();

                for (id, prop) in props {
                    self.props.insert(id, prop);
                    ids.push(id);
                }

                (!ids.is_empty()).then(|| Action::RemoveProps(ids))
            }

            Action::RemoveSolids(ids) => {
                let mut solids = Vec::new();
                for id in ids {
                    solids.push((id, self.solids.remove(&id).unwrap()));
                }

                (!solids.is_empty()).then(|| Action::AddSolids(solids))
            }

            Action::RemoveProps(ids) => {
                let mut props = Vec::new();
                for id in ids {
                    props.push((id, self.props.remove(&id).unwrap()));
                }

                (!props.is_empty()).then(|| Action::AddProps(props))
            }

            Action::SelectSolids(ids) => {
                for id in &ids {
                    let solid = self.solids.get_mut(id).unwrap();
                    solid.set_selected(!solid.selected());
                    solid.recalc(ctx.graphics);
                }
                (!ids.is_empty()).then(|| Action::SelectSolids(ids))
            }

            Action::SelectFaces(locators) => {
                for locator in &locators {
                    let solid = self.solids.get_mut(&locator.solid).unwrap();
                    solid.set_face_selected(locator.face, !solid.face_selected(locator.face));
                    solid.recalc(ctx.graphics);
                }
                (!locators.is_empty()).then(|| Action::SelectFaces(locators))
            }

            Action::SelectPoints(locators) => {
                for locator in &locators {
                    let solid = self.solids.get_mut(&locator.solid).unwrap();
                    solid.set_point_selected(locator.point, !solid.point_selected(locator.point));
                    solid.recalc(ctx.graphics);
                }
                (!locators.is_empty()).then(|| Action::SelectPoints(locators))
            }

            Action::SelectProps(ids) => {
                for id in &ids {
                    let prop = self.props.get_mut(id).unwrap();
                    prop.set_selected(!prop.selected());
                    prop.recalc(ctx.graphics);
                }
                (!ids.is_empty()).then(|| Action::SelectProps(ids))
            }

            Action::SelectAll(kind) => match kind {
                ElementKind::Solid => {
                    let selected = self
                        .solids
                        .iter()
                        .filter_map(|(id, solid)| solid.selected().then(|| *id))
                        .collect::<Vec<_>>();

                    let len_max = self.solids.len();
                    let mut ids = Vec::new();

                    for (id, solid) in &mut self.solids {
                        if selected.is_empty() || selected.len() == len_max {
                            solid.set_selected(!solid.selected());
                            solid.recalc(ctx.graphics);
                            ids.push(*id);
                        } else {
                            solid.set_selected(true);
                            if !selected.contains(id) {
                                solid.recalc(ctx.graphics);
                                ids.push(*id);
                            }
                        }
                    }

                    (!ids.is_empty()).then(|| Action::SelectSolids(ids))
                }
                ElementKind::Prop => {
                    let selected = self
                        .props
                        .iter()
                        .filter_map(|(id, prop)| prop.selected().then(|| *id))
                        .collect::<Vec<_>>();

                    let len_max = self.props.len();
                    let mut ids = Vec::new();

                    for (id, prop) in &mut self.props {
                        if selected.is_empty() || selected.len() == len_max {
                            prop.set_selected(!prop.selected());
                            prop.recalc(ctx.graphics);
                            ids.push(*id);
                        } else {
                            prop.set_selected(true);
                            if !selected.contains(id) {
                                prop.recalc(ctx.graphics);
                                ids.push(*id);
                            }
                        }
                    }

                    (!ids.is_empty()).then(|| Action::SelectProps(ids))
                }
                _ => panic!("Select all is undefined on faces and points"),
            },

            Action::DeselectAll(kind) => match kind {
                ElementKind::Solid => {
                    let mut ids = Vec::new();
                    for (id, solid) in &mut self.solids {
                        if solid.selected() {
                            solid.set_selected(false);
                            solid.recalc(ctx.graphics);
                            ids.push(*id);
                        }
                    }
                    (!ids.is_empty()).then(|| Action::SelectSolids(ids))
                }
                ElementKind::Face => {
                    let mut locators = Vec::new();

                    for (sid, solid) in &mut self.solids {
                        let mut recalc = false;

                        for fid in 0..6 {
                            if solid.face_selected(fid) {
                                solid.set_face_selected(fid, false);
                                locators.push(FaceLocator {
                                    solid: *sid,
                                    face: fid,
                                });
                                recalc = true;
                            }
                        }

                        if recalc {
                            solid.recalc(ctx.graphics);
                        }
                    }

                    (!locators.is_empty()).then(|| Action::SelectFaces(locators))
                }
                ElementKind::Point => {
                    let mut locators = Vec::new();

                    for (sid, solid) in &mut self.solids {
                        let mut recalc = false;

                        for pid in 0..8 {
                            if solid.point_selected(pid) {
                                solid.set_point_selected(pid, false);
                                locators.push(PointLocator {
                                    solid: *sid,
                                    point: pid,
                                });
                                recalc = true;
                            }
                        }

                        if recalc {
                            solid.recalc(ctx.graphics);
                        }
                    }

                    (!locators.is_empty()).then(|| Action::SelectPoints(locators))
                }
                ElementKind::Prop => {
                    let mut ids = Vec::new();
                    for (pid, prop) in &mut self.props {
                        if prop.selected() {
                            prop.set_selected(false);
                            prop.recalc(ctx.graphics);
                            ids.push(*pid);
                        }
                    }
                    (!ids.is_empty()).then(|| Action::SelectProps(ids))
                }
            },

            Action::Move { kind, delta } => match kind {
                ElementKind::Solid | ElementKind::Face | ElementKind::Point => {
                    let mut changed = false;
                    for solid in self.solids.values_mut() {
                        if solid.displace(delta, kind) {
                            solid.recalc(ctx.graphics);
                            changed = true;
                        }
                    }
                    changed.then(|| Action::Move {
                        kind,
                        delta: -delta,
                    })
                }
                ElementKind::Prop => {
                    let mut changed = false;
                    for prop in self.props.values_mut().filter(|prop| prop.selected()) {
                        if prop.displace(delta, ElementKind::Prop) {
                            prop.recalc(ctx.graphics);
                            changed = true;
                        }
                    }
                    changed.then(|| Action::Move {
                        kind: ElementKind::Prop,
                        delta: -delta,
                    })
                }
            },

            Action::RotateProps(quat) => {
                let mut changed = false;
                for prop in self.props.values_mut().filter(|prop| prop.selected()) {
                    prop.set_rotation(quat * prop.rotation());
                    prop.recalc(ctx.graphics);
                    changed = true;
                }

                changed.then(|| Action::RotateProps(quat.invert()))
            }

            Action::SetPropRotations(rotations) => {
                let rotations = rotations
                    .into_iter()
                    .map(|(index, quat)| {
                        let prop = self.props.get_mut(&index).unwrap();
                        let old = prop.rotation();
                        prop.set_rotation(quat);
                        prop.recalc(ctx.graphics);
                        (index, old)
                    })
                    .collect::<Vec<_>>();

                (!rotations.is_empty()).then(|| Action::SetPropRotations(rotations))
            }

            Action::AssignTexture(texture) => {
                let mut changes = Vec::new();
                for (sid, solid) in &mut self.solids {
                    for fid in 0..6 {
                        if solid.face_selected(fid) {
                            let old = solid.retexture(fid, texture);
                            if old != texture {
                                solid.recalc(ctx.graphics);
                                changes.push((
                                    FaceLocator {
                                        solid: *sid,
                                        face: fid,
                                    },
                                    old,
                                ))
                            }
                        }
                    }
                }
                (!changes.is_empty()).then(|| Action::AssignTextures(changes))
            }

            Action::AssignTextures(textures) => {
                let mut changes = Vec::new();
                for (locator, texture) in textures {
                    let solid = self.solids.get_mut(&locator.solid).unwrap();
                    let old = solid.retexture(locator.face, texture);
                    if old != texture {
                        solid.recalc(ctx.graphics);
                        changes.push((locator, old));
                    }
                }
                (!changes.is_empty()).then(|| Action::AssignTextures(changes))
            }

            Action::DeleteSolids => {
                let ids = self
                    .solids
                    .iter()
                    .filter(|(_, solid)| solid.selected())
                    .map(|(id, _)| *id)
                    .collect::<Vec<_>>();

                let mut solids = Vec::new();

                for id in ids {
                    let solid = self.solids.remove(&id).unwrap();
                    solids.push((id, solid));
                }

                (!solids.is_empty()).then(|| Action::AddSolids(solids))
            }

            Action::DeleteProps => {
                let ids = self
                    .props
                    .iter()
                    .filter(|(_, prop)| prop.selected())
                    .map(|(id, _)| *id)
                    .collect::<Vec<_>>();

                let mut props = Vec::new();

                for id in ids {
                    let prop = self.props.remove(&id).unwrap();
                    props.push((id, prop));
                }

                (!props.is_empty()).then(|| Action::AddProps(props))
            }

            Action::RotateSolids {
                axis,
                iters,
                reverse,
                snap,
            } => {
                if let Some(center) = self.calc_center(ElementKind::Solid) {
                    let center = center.round(snap);
                    let mut changed = false;
                    for solid in self.solids.values_mut().filter(|solid| solid.selected()) {
                        solid.rotate(center, axis, iters, reverse);
                        solid.recalc(ctx.graphics);
                        changed = true;
                    }

                    changed.then(|| Action::UnrotateSolids {
                        axis,
                        iters,
                        reverse: !reverse,
                        center,
                    })
                } else {
                    None
                }
            }

            Action::UnrotateSolids {
                axis,
                iters,
                reverse,
                center,
            } => {
                let mut changed = false;
                for solid in self.solids.values_mut().filter(|solid| solid.selected()) {
                    solid.rotate(center, axis, iters, reverse);
                    solid.recalc(ctx.graphics);
                    changed = true;
                }

                changed.then(|| Action::UnrotateSolids {
                    axis,
                    iters,
                    reverse: !reverse,
                    center,
                })
            }

            Action::ReplaceSolids { ids, solids } => {
                let mut old_solids = Vec::new();
                let mut new_ids = Vec::new();
                for id in ids {
                    old_solids.push((id, self.solids.remove(&id).unwrap()));
                }

                for solid in solids {
                    let id = self.next_elem_id;
                    self.next_elem_id += 1;
                    self.solids.insert(id, solid);
                    new_ids.push(id);
                }

                (!old_solids.is_empty()).then(|| Action::ReplaceSolidsExact {
                    ids: new_ids,
                    solids: old_solids,
                })
            }

            Action::ReplaceSolidsExact { ids, solids } => {
                let mut old_solids = Vec::new();
                let mut new_ids = Vec::new();
                for id in ids {
                    old_solids.push((id, self.solids.remove(&id).unwrap()));
                }

                for (id, solid) in solids {
                    self.solids.insert(id, solid);
                    new_ids.push(id);
                }

                (!old_solids.is_empty()).then(|| Action::ReplaceSolidsExact {
                    ids: new_ids,
                    solids: old_solids,
                })
            }
        }
    }
}

pub struct Context<'a> {
    pub graphics: &'a Graphics,
}

pub enum Action {
    NewSolids(Vec<Solid>),
    NewProps(Vec<Prop>),

    AddSolids(Vec<(usize, Solid)>),
    AddProps(Vec<(usize, Prop)>),

    RemoveSolids(Vec<usize>),
    RemoveProps(Vec<usize>),

    SelectSolids(Vec<usize>),
    SelectFaces(Vec<FaceLocator>),
    SelectPoints(Vec<PointLocator>),
    SelectProps(Vec<usize>),
    SelectAll(ElementKind),
    DeselectAll(ElementKind),

    Move {
        kind: ElementKind,
        delta: Vector3<i32>,
    },

    RotateProps(Quaternion<f32>),
    SetPropRotations(Vec<(usize, Quaternion<f32>)>),
    AssignTexture(TextureID),
    AssignTextures(Vec<(FaceLocator, TextureID)>),

    DeleteSolids,
    DeleteProps,
    RotateSolids {
        axis: Axis,
        iters: u32,
        reverse: bool,
        snap: i32,
    },
    UnrotateSolids {
        axis: Axis,
        iters: u32,
        reverse: bool,
        center: Vector3<i32>,
    },
    ReplaceSolids {
        ids: Vec<usize>,
        solids: Vec<Solid>,
    },
    ReplaceSolidsExact {
        ids: Vec<usize>,
        solids: Vec<(usize, Solid)>,
    },
}

struct UndoStack {
    vec: Vec<Action>,
}

impl Default for UndoStack {
    fn default() -> Self {
        Self {
            vec: Vec::with_capacity(64),
        }
    }
}

impl UndoStack {
    fn push(&mut self, value: Action) {
        if self.vec.len() == 64 {
            self.vec.remove(0);
        }
        self.vec.push(value);
    }

    fn pop(&mut self) -> Option<Action> {
        self.vec.pop()
    }
}
