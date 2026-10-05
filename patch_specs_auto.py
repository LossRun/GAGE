import os
import glob

target_file = None
for root, _, files in os.walk("src"):
    for file in files:
        if file.endswith(".rs"):
            p = os.path.join(root, file)
            with open(p, "r", errors="ignore") as f:
                content = f.read()
            if "test_boolean_logic" in content:
                target_file = p
                break
    if target_file:
        break

if not target_file:
    print("Could not find any file containing 'test_boolean_logic' in src/")
    exit(1)

print(f"Found target test file: {target_file}")

with open(target_file, "r") as f:
    text = f.read()

start_marker = '"test_boolean_logic"'
end_marker = '"test_step_physics"'

start_idx = text.find(start_marker)
open_bracket = text.rfind('[', 0, start_idx)

end_idx = text.find(end_marker, start_idx)
close_bracket = text.find(']', end_idx)

if open_bracket != -1 and close_bracket != -1:
    new_specs_list = [
        "test_boolean_logic",
        "test_canvas_buffer",
        "test_control_flow",
        "test_dynamic_arrays",
        "test_file_io",
        "test_functions_recursion",
        "test_math_and_mat4",
        "test_oop_classes",
        "test_primitives_and_types",
        "test_sim_primitives",
        "test_simd_shading",
        "test_simd_vectors",
        "test_step_physics",
        "test_extension_matrix",
        "test_logic_tristate",
        "test_event_priority_queue",
        "test_rk4_integrator",
        "test_physics_restitution",
        "test_grid_toroidal_wrap",
        "test_grid_still_life",
        "test_gate_propagation_delay",
        "test_braille_canvas_subpixels",
        "test_field_gravity_inversion",
        "test_collision_tunneling",
        "test_waveform_sampler",
        "test_simd_batch_operations",
        "test_nbody_gravity_slingshot",
        "test_zero_delay_convergence",
    ]
    replacement = "[\n" + "\n".join([f'        "{s}",' for s in new_specs_list]) + "\n    "
    text = text[:open_bracket] + replacement + text[close_bracket:]
    with open(target_file, "w") as f:
        f.write(text)
    print(f"✔ Successfully updated {target_file} to 28 language specifications!")
else:
    print(f"Brackets not found in {target_file}")
