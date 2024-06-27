# safevec

Small Rust implementation of generational-indexed Vec without any dependencies.

Data is stored contigiously.
Element removal is implemented like swap_remove internally, therefore there are no holes and iteration speed is maximized due to higher CPU cache utilization.
Element push returns special index that can be used to safely access data across element removals.

Use cases include using it as a component storage for ECS game engine implementation.
