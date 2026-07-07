---
# 📋 Documentação: {{name}}
---

## 🏗️ Diagrama da Estrutura

Diagramas gerados automaticamente para retratar a hierarquia das classes

{{#if structs}}

```mermaid
graph TD;
    {{#each structs}}
    {{name}} --> {{this.name}};
    click {{name}} "./{{path}}{{name}}"
    {{/each}}

```

{{else}}
:::note
se pá que não teve diagrama
:::
{{/if}}

---

## 🔍 Variáveis e Campos

Campos definidos nas estruturas encontradas no módulo:

{{#if fields}}

<table>
  <thead>
    <tr>
      <th>Nome do Campo</th>
      <th>Tipo de Dado</th>
    </tr>
  </thead>
  <tbody>
    {{#each fields}}
    <tr>
      <td>{{this.name}}</td>
      <td>{{this.data_type}}</td>
    </tr>
    {{/each}}
  </tbody>
</table>

{{else}}
:::note
se pá que não tem nenhuma variável
:::
{{/if}}

---

## ⚙️ Funções do Módulo

Lista de funções identificadas e suas respectivas assinaturas:

{{#if functions}}

{{#each functions}}
**Comentário**: {{this.comment}}

```rust
{{{this.signature}}}
```

{{/each}}
{{else}}
:::note
se pá que não tem nenhuma função
:::
{{/if}}

---

## 📂 Código-Fonte Completo

```rust title="{{path}}"
{{{source_code}}}

```
