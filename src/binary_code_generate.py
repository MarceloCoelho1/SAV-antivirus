import random

# Tamanho das linhas e número de linhas
linha_tamanho = 200
num_linhas = 200

# Gere um arquivo binário aleatório
with open("binary_code.txt", "wb") as arquivo:
    for _ in range(num_linhas):
        linha_binaria = "".join(random.choice("01") for _ in range(linha_tamanho))
        linha_bytes = bytes([int(linha_binaria[i:i+8], 2) for i in range(0, len(linha_binaria), 8)])
        arquivo.write(linha_bytes + b'\n')
