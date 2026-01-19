#include <chrono>
#include <iostream>
#include <vector>

int main(void) {
    int N = 1000;
    std::vector<std::vector<int>> matA(N, std::vector<int>(N, 1));
    std::vector<std::vector<int>> matB(N, std::vector<int>(N, 2));
    std::vector<std::vector<int>> result(N, std::vector<int>(N, 0));

    auto start = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < N; ++i)
        for (int j = 0; j < N; ++j)
            for (int k = 0; k < N; ++k)
                result[i][j] += matA[i][k] * matB[k][j];

    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;

    std::cout << "Elapsed time: " << elapsed.count() << " seconds" << std::endl;

    return 0;
}
