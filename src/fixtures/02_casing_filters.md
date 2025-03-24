---
args:
  - QUERY
edit:
  - "src/main.rs"
---

- Create a new file called `src/component/{{ QUERY | kebab }}.tsx`
- Add a React component in that file which calls the `use{{ QUERY | pascal }}Query()` hook and renders the result
- Render some text above the result saying "Response from {{ QUERY | camel }}"

Snake case for the hell of it: {{ QUERY | snake }}
