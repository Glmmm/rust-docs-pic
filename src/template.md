---
# 📋 Documentação: {{name}}
---

## 🏗️ Diagrama de Estrutura

Diagramas gerados automaticamente para retratar a hierarquia das classes

<!-- TODO: adicionar interação click para diagramas

```mermaid
flowchart LR
    A->B
    B->C
    C->D
    click A callback "Tooltip for a callback"
    click B "https://www.github.com" "This is a tooltip for a link"
    click C call callback() "Tooltip for a callback"
    click D href "https://www.github.com" "This is a tooltip for a link"
```
--->

```mermaid
graph TD;
    {{#each structs}}
    {{name}} --> {{this.name}};
    {{/each}}

```

---

## 🔍 Variáveis e Campos

Campos definidos nas estruturas encontradas no módulo:

| Nome do Campo    | Tipo de Dado       |
| ---------------- | ------------------ |
| {{#each fields}} |                    |
| `{{this.name}}`  | {{this.data_type}} |
| {{/each}}        |                    |

---

## ⚙️ Funções do Módulo

Lista de funções identificadas e suas respectivas assinaturas:

{{#each functions}}

### `{{this.signature}}`

**Comentário**: {{this.comment}}

---

{{/each}}

<!-- TODO: adicinar titulo (path) do código ou accordion

```js:app.js
console.log("Hello, world!");
```

```
// app.js
console.log("Hello, world!");
```
-->

## 📂 Código-Fonte Completo

```rust
{{{source_code}}}

```

---
