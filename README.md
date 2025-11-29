# Rust Compute Demo

![Rust](https://img.shields.io/badge/rust-1.91.1-orange)
![Axum](https://img.shields.io/badge/axum-0.7-blue)
![Tokio](https://img.shields.io/badge/tokio-1-purple)
![Rayon](https://img.shields.io/badge/rayon-1.10-lightgrey)
![Tailwind](https://img.shields.io/badge/tailwindcss-latest-blue)
![Render](https://img.shields.io/badge/render-deployed-brightgreen)

**Interactive number computation demo with parallel processing**

> Computes large sets of random numbers asynchronously and displays the results with a dynamic progress bar.  
> Utilizes parallel processing to handle millions of numbers efficiently.

---

## Endpoints

| Path        | Method | Description                                               |
|-------------|--------|-----------------------------------------------------------|
| `/`         | `GET`  | Main page with buttons to start computation and progress |
| `/compute`  | `GET`  | Returns the computation result for a specified number of random numbers |

Query parameter for `/compute`:

- `size` – Number of random numbers to generate and process (default: 10,000,000)

---

## Features

- **Dynamic computation** of large number sets with a progress bar  
- **Parallel processing** using Rayon for faster computation  
- Interactive front-end using **Tailwind CSS**  
- Fully asynchronous backend with **Axum** and **Tokio**  
- Customizable computation size via query parameters  
- Simple, responsive, and user-friendly UI  

---

## Technology Stack

- **Rust 1.91.1** – systems programming language  
- **Axum 0.7** – lightweight web framework for Rust  
- **Tokio 1** – asynchronous runtime for Rust  
- **Rayon 1.10** – data parallelism library for high-performance computation  
- **Tailwind CSS** – utility-first responsive UI framework  
- **Render.com** – zero-config deployment for Rust web apps  

---

## Deploy in 10 seconds

[![Deploy to Render](https://render.com/images/deploy-to-render-button.svg)](https://render.com/deploy)
