use timewise_analytics::*;

fn main() {
    let x = [1.0, 2.0, 3.0, 4.0, 5.0];
    let y = [2.0, 4.0, 6.0, 8.0, 10.0];

    match regressao_linear(&x, &y) {
        Some((a, b)) => {
            println!("Regressão linear encontrada: y = {:.2}x + {:.2}", a, b);

            let previsao = prever(&x, (a, b));
            println!("Valores previstos: {:?}", previsao);

            let mse = calcular_mse(&y, &previsao);
            let r2 = calcular_r2(&y, &previsao);

            println!("MSE: {:.4}", mse);
            println!("R²: {:.4}", r2);
        }
        None => {
            println!("Não foi possível calcular a regressão linear.");
        }
    }
}
