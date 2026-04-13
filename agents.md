* The engine is currently in a very early stage of development. The absence of certain features, or their incomplete implementation, is not intentional — it simply reflects that development is still ongoing.

## Agent Behavior

- Make the **smallest possible change** to solve the problem
- Change low-level code freely; minimize changes to high-level/public API
- Do **not** remove, rename, or refactor code that wasn't part of the request
- Do **not** delete public API methods — suggest removal instead and let the developer decide
- If a change at a lower layer makes a higher-layer construct redundant, **point it out** rather than removing it