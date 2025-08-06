# 📊 Análise de Séries Temporais com Regressão Linear em Rust

Este projeto foi desenvolvido como parte do meu portfólio para a disciplina de Estrutura de Dados. A proposta era criar uma solução simples, porém funcional, capaz de realizar **análise de séries temporais** com base em **regressão linear pura** — tudo isso usando a linguagem Rust e **sem bibliotecas externas**.

---

## 🎯 Propósito

Imagine uma empresa fictícia chamada **TimeWise Analytics**, que precisa de uma ferramenta para prever dados com base em séries temporais, como temperaturas, vendas, consumo, etc. A ideia aqui é construir um módulo que:

- Descubra a linha reta que melhor representa os dados (regressão linear);
- Calcule o quão boa é essa linha com duas métricas bem conhecidas: **MSE** e **R²**;
- E por fim, use essa linha para prever valores futuros com base em novos dados.

---

## 🧩 Como está organizado

- `lib.rs`: Aqui está a alma do projeto. Todas as funções matemáticas estão neste arquivo:
  - `regressao_linear`: encontra os coeficientes `a` e `b` da reta `y = ax + b`.
  - `calcular_mse`: mostra o erro médio entre o real e o previsto.
  - `calcular_r2`: diz o quanto o modelo explica os dados.
  - `prever`: gera os valores previstos a partir da reta.

- `main.rs`: Um exemplo simples rodando com números fictícios (mas reais o suficiente pra mostrar tudo funcionando).

---

## ⚙️ Como executar o projeto

Se você já tem Rust instalado, é só seguir:

```bash
cargo run

