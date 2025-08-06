/// Realiza regressão linear simples y = ax + b
pub fn regressao_linear(x: &[f64], y: &[f64]) -> Option<(f64, f64)> {
    let n = x.len();
    if n != y.len() || n == 0 {
        return None;
    }

    let soma_x: f64 = x.iter().sum();
    let soma_y: f64 = y.iter().sum();
    let soma_xy: f64 = x.iter().zip(y).map(|(xi, yi)| xi * yi).sum();
    let soma_x2: f64 = x.iter().map(|xi| xi * xi).sum();

    let n_f = n as f64;
    let numerador = n_f * soma_xy - soma_x * soma_y;
    let denominador = n_f * soma_x2 - soma_x * soma_x;

    if denominador == 0.0 {
        return None;
    }

    let a = numerador / denominador;
    let b = (soma_y - a * soma_x) / n_f;

    Some((a, b))
}

/// Calcula o erro quadrático médio (MSE)
pub fn calcular_mse(y_verdadeiro: &[f64], y_predito: &[f64]) -> f64 {
    let n = y_verdadeiro.len();
    if n == 0 || n != y_predito.len() {
        return f64::NAN;
    }

    y_verdadeiro.iter().zip(y_predito)
        .map(|(y, y_hat)| (y - y_hat).powi(2))
        .sum::<f64>() / n as f64
}

/// Calcula o coeficiente de determinação (R²)
pub fn calcular_r2(y_verdadeiro: &[f64], y_predito: &[f64]) -> f64 {
    let media_y = y_verdadeiro.iter().sum::<f64>() / y_verdadeiro.len() as f64;
    let ss_total: f64 = y_verdadeiro.iter().map(|y| (y - media_y).powi(2)).sum();
    let ss_res: f64 = y_verdadeiro.iter().zip(y_predito).map(|(y, y_hat)| (y - y_hat).powi(2)).sum();

    1.0 - ss_res / ss_total
}

/// Previsões com os coeficientes da regressão
pub fn prever(x: &[f64], coeficientes: (f64, f64)) -> Vec<f64> {
    let (a, b) = coeficientes;
    x.iter().map(|xi| a * xi + b).collect()
}
