import subprocess
from concurrent.futures.process import ProcessPoolExecutor

# 1 => 45
# 2 => 40s
# 3 => 40s

PROCESS_COUNT = 1
THREAD_COUNT = 3

def main() -> None:
    with ProcessPoolExecutor() as executor:
        executor.map(attack, range(PROCESS_COUNT))


def attack(i: int) -> None:
    p = subprocess.run(["./target/release/aes-attacks", "81d6cdc3bd16fb8d72b9bb88818b5be9", "eff93508630187b8d3494e8b70e6887e", str(THREAD_COUNT), str(PROCESS_COUNT), str(i)], capture_output=True)
    print(p.stdout.decode().strip())


if __name__ == '__main__':
    main()
