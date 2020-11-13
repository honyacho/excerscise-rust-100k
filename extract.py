import gensim
import gzip

def main():
    lines = []
    with open('countries.txt','r') as f:
        lines = f.readlines()
    model = gensim.models.KeyedVectors.load_word2vec_format('GoogleNews-vectors-negative300.bin', binary=True)
    lines = [l.rstrip() for l in lines]
    with open('out.txt', 'w') as ofile:
        for l in lines:
            try:
                vec = model[l]
                ofile.write("{}\n".format(l))
                ofile.write("{}\n".format(" ".join(map(str, vec))))
            except:
                "do nothing"

if __name__ == '__main__':
    main()