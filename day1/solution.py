def main():
    with open("input", 'r') as f:
        total = 0
        for line in f.readlines():
            line.strip()
            if line.startswith('+'):
                n = int(line[1:])
            else:
                n = -1*int(line[1:])
            total += n

        print total

if __name__ == '__main__':
    main()
