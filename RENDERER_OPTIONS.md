# 3D Rendering Options for Dimension

## Project snapshot

Dimension currently uses `eframe` and `egui` for its desktop interface. The lockfile resolves `eframe` to 0.29.1, and `src/proj_editor/main.rs` is a stub, so there is not yet a 3D renderer to preserve or work around.

In this version, eframe's default graphics backend is Glow (OpenGL). Eframe can also be built with its WGPU backend. Newer eframe tutorials may assume different defaults, so check the version shown in a tutorial before copying its setup. The eframe changelog records the later switch to WGPU as the default in version 0.34.

## Shortlist

| Option | What it is | How it fits Dimension |
| --- | --- | --- |
| **Glow / OpenGL** | A relatively direct way to issue OpenGL drawing commands. | The quickest route to a 3D preview inside the current eframe app. Egui's official custom 3D example shows rendering into part of an egui window. The API is low-level, and the example uses `unsafe` code. |
| **WGPU** | A portable, safe Rust graphics API, rather than a ready-made scene renderer. | A strong foundation for a renderer that Dimension owns. Eframe supports custom WGPU drawing inside egui, but setup and GPU concepts take more learning. |
| **three-d** | A higher-level 3D renderer with abstractions for common scene and drawing tasks. | Potentially simpler for drawing a scene than raw OpenGL or WGPU. Its egui support is not a drop-in renderer panel for this existing eframe app, so combining the two may take extra integration work. |
| **Bevy** | A full game engine with rendering, assets, and other engine systems. | Consider it if the main goal becomes making games rather than building Dimension. `bevy_egui` puts egui inside a Bevy app, which means changing the app's host rather than simply adding a renderer to eframe. |

## Recommendation

For the fastest first preview in Dimension, start with Glow and adapt the official eframe 3D example. It matches the project's current backend and lets the editor remain an eframe app.

If you already know that a custom renderer is a central part of Dimension, consider starting with WGPU instead. It takes more setup, but choosing it early avoids building a large renderer around OpenGL and then needing to replace it. Eframe exposes WGPU rendering callbacks for drawing in an egui region.

Treat three-d as an option for a small separate experiment if its higher-level scene API appeals to you. It may make 3D drawing simpler, but it will take extra work to fit into Dimension's current eframe viewport.

## A manageable first milestone

1. Add a large preview area to the project editor.
2. Draw a triangle and make it rotate.
3. Turn it into a cube, then add a movable camera and depth testing.
4. Add model loading, lighting, and materials one at a time.

Keeping the preview inside the editor window will make it easier to add scene panels and tools around it later.

## Official references

- [Egui custom 3D example using Glow](https://github.com/emilk/egui/blob/main/examples/custom_3d_glow/src/main.rs)
- [Egui custom 3D example using WGPU](https://github.com/emilk/egui/blob/main/crates/egui_demo_app/src/apps/custom3d_wgpu.rs)
- [three-d documentation](https://docs.rs/three-d/latest/three_d/)
- [bevy_egui documentation](https://docs.rs/bevy_egui/latest/bevy_egui/)
- [Eframe changelog](https://github.com/emilk/egui/blob/main/crates/eframe/CHANGELOG.md)
