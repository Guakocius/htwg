#include <cstring>
#include <iostream>
#include <stdint.h>
#include <algorithm>

#define PWD_LEN 6

/**
 * Bitwise rotation by n bits
 * @param n rotation amount
 * @param t value being rotated
 * @return Bitwise rotated unsigned int
 */
/*inline uint S(uint n, uint t ) {
  return (t << n) | (t >> (32 - n));
}*/
/*uint swap(uint x) {
  return ((x & 0xFF) << 24) | ((x & 0xFF00) << 8) | ((x & 0xFF0000) >> 8) | ((x & 0xFF000000) >> 24);
}*/

/*std::vector<uint> hash_to_words(const std::string& s) {
  std::vector<uint> W(80);

  for (int i = 0; i < 16; ++i) { // i=0..127
    W[i] =  ((uint)(unsigned char)s[i*4] << 24) 
            | ((uint) (unsigned char)s[i*4+1] << 16) 
            | ((uint)(unsigned char)s[i*4+2] << 8) 
            | ((uint)(unsigned char)s[i*4+3]); 
  } 
  for (int t = 16; t < 80; ++t) {
    W[t] = S(1, W[t-3] ^ W[t-8] ^ W[t-14] ^ W[t-16]);
  }
  return W;
}*/

/**
 * K : The four constants depending of t
 * @param t the iterator 0..79
 * @return K(t)
 */
/*uint K(int t) {
  if (t >= 0 && t <= 19) return 0x5A827999;
  else if (t >= 20 && t <= 39) return 0x6ED9EBA1;
  else if (t >= 40 && t <= 59) return 0x8F1BBCDC;
  else return 0xCA62C1D6;
}*/

/**
* 80 word sequence
*/
/*uint f(const uint t, uint B, uint C, uint D) {
  if (t >= 0 && t <= 19) return (B & C) | ((~B) & D);
  else if (t >= 20 && t <= 39) return B ^ C ^ D;
  else if (t >= 40 && t <= 59) return (B & C) | (B & D) | (C & D);
  else return B ^ C ^ D;
}*/

/*std::string digest_hex(uint x) {
  std::ostringstream digest;
  digest << std::hex << std::setfill('0') << std::setw(8) << x;
  return digest.str();

}*/

/*std::string compress(std::vector<uint>& W) {

  uint H0 = 0x67452301, H1 = 0xEFCDAB89, H2 = 0x98BADCFE, H3 = 0x10325476, H4 = 0xC3D2E1F0; // H values for compressing
  uint A,B,C,D,E,TEMP; // first 5-word buffer
  A = H0, B = H1, C = H2, D = H3, E = H4;

  for (int t = 0; t < 80; t++) {
    TEMP = S(5, A) + f(t, B, C, D) + E + W[t] + K(t);
    E = D, D = C, C = S(30, B), B = A, A = TEMP;
  }
  H0 += A, H1 += B, H2 += C, H3 += D, H4 += E;
  std::string digest;
  digest.reserve(40); // 160 / 4 = 40 Hex digits
  digest += digest_hex(H0);
  digest += digest_hex(H1);
  digest += digest_hex(H2);
  digest += digest_hex(H3);
  digest += digest_hex(H4);

  return digest;
}*/
/**
 * Optimized version with lower runtime
 */

const char charset[] = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const int CHARS = 62;


void compress(uint* H, uint* W) {
  uint32_t A = H[0], B = H[1], C = H[2], D = H[3], E = H[4];
  uint32_t TEMP;

  for (int t = 0; t < 80; t++) {
    TEMP =  ((A << 5) | (A >> (32 - 5)))
            + ((t < 20) ? ((B & C) | (~B & D)) :
              (t < 40) ? (B ^ C ^ D) :
              (t < 60) ? ((B & C) | (B & D) | (C & D)) :
              (B ^ C ^ D))
            + E + W[t]
            + (t < 20 ? 0x5A827999 :
               t < 40 ? 0x6ED9EBA1 :
               t < 60 ? 0x8F1BBCDC :
                        0xCA62C1D6);

    E = D, D = C, C = (B >> 2) | (B << 30), B = A, A = TEMP;
  }

  H[0] += A, H[1] += B, H[2] += C, H[3] += D, H[4] += E;
}

bool next_pwd(char* pwd) {
  int j = PWD_LEN-1;

  while (j >= 0) {
    const char* p = std::find(std::begin(charset), std::end(charset), pwd[j]);

    int idx = p - charset; // curr idx
    if (idx + 1 < CHARS) {
      pwd[j] = charset[idx+1];
      return true;
    }
    pwd[j] = charset[0]; // reset buf
    j--;
  }
  return false;
}

/*std::string hex(const std::string& input) {
  std::ostringstream s;
  s << std::hex << std::setfill('0');

  for (unsigned char c : input) {
    s << std::setw(2) << static_cast<int>(c);
  }
  return s.str();
}*/

/*std::string rand_pwd() {
  std::string pwd_dir, pwd;
  pwd_dir.reserve(26 * 2 + 10);
  pwd.reserve(PWD_LEN);

  for (char c = '0'; c <= '9'; ++c) pwd_dir += c;
  for (char c = 'A'; c <= 'Z'; ++c) pwd_dir += c;
  for (char c = 'a'; c <= 'z'; ++c) pwd_dir += c;

  std::random_device rd;
  std::mt19937 gen(rd());
  std::uniform_int_distribution<> dist(0, pwd_dir.size() - 1);

  for (int i = 0; i < PWD_LEN; i++) {
      pwd += pwd_dir[dist(gen)];
    }

    return pwd;
}*/

/*std::string pad(const std::string& m) {
  std::string pad = m;

  const unsigned long long bit_len = (const unsigned long long)m.size() * 8;

  pad.push_back((char)0x80);

  while ((pad.size() % 64) != 56) pad.push_back(0); // 64 - 8 = 56

  for (int i = 7; i >= 0; i--) pad.push_back((bit_len >> (i * 8)) & 0xFF);

  return pad;
}*/

/*std::string sha1_decrypt(const std::string& m, const std::string& h) {
  
  std::string pwd_hashed, padding;

  std::cout << "random password: " << pwd << std::endl;
  padding = pad(pwd);

  uint W[80]; 
  for (int t = 16; t < 80; t++) {
    W[t]  = ((W[t-3] ^ W[t-8] ^ W[t-14] ^ W[t-16]) << 1)
          | ((W[t-3] ^ W[t-8] ^ W[t-14] ^ W[t-16]) >> 31);
  }
  std::string hash = compress(W);

  return hash;

}*/

inline bool hash_cmp(const uint32_t* H, const uint32_t* t) {
  return (H[0] == t[0] && H[1] == t[1] && H[2] == t[2] && H[3] == t[3] && H[4] == t[4]);
}

int main(void) {
  std::string hash = "dbc3337f151da4276572aaaa424cddb0d89a5422";

  uint8_t block[64];
  uint32_t W[80];
  uint32_t H[5];
  uint32_t target[5] = {
    0xdbc3337f,  0x151da427, 0x6572aaaa, 0x424cddb0, 0xd89a5422
  };

  char pwd[PWD_LEN+1] = "000000";
  
  std::string pwd_hash, pwd_encrypt;
  uint32_t i = 0;

  while (1) {
    // reset block
    memset(block, 0, 64);
    memcpy(block, pwd, PWD_LEN);
    block[PWD_LEN] = 0x80;

    uint64_t bit_len = PWD_LEN << 3; // 6 * 8

    for (int i = 0; i < 8; i++) block[63 - i] = (bit_len >> (i*8)) & 0xFF;

    // load block to W
    for (int i = 0; i < 16; i++) {
      W[i] =  (block[i*4] << 24) | (block[i*4+1] << 16) |
              (block[i*4+2] << 8) | block[i*4+3];
    }
    for (int t = 16; t < 80; t++) {
      W[t] = ((W[t-3] ^ W[t-8] ^ W[t-14] ^ W[t-16]) << 1) |
             ((W[t-3] ^ W[t-8] ^ W[t-14] ^ W[t-16]) >> 31);
    }

    // init H
    H[0] = 0x67452301; H[1] = 0xEFCDAB89;
    H[2] = 0x98BADCFE; H[3] = 0x10325476;
    H[4] = 0xC3D2E1F0;
    compress(H, W);

    std::cout << "TRIED PASSWORD: " << pwd << "\n";

    if (hash_cmp(H, target)) {
      std::cout << "\nFOUND PASSWORD: " << pwd << "\nAFTER " << i << " SEARCHES\n";
      break;
    }
    
    if (!next_pwd(pwd)) {
      std::cout << "ERR: Exhausted search space\n";
      break;
    }
    i++;
  }
  return 0;
}
