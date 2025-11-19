#include <string>
#include <iomanip>
#include <iostream>
#include <random>

#define uint unsigned int
#define K0  0x5A827999
#define K20 0x6ED9EBA1
#define K40 0x8F1BBCDC
#define K60 0xCA62C1D6

#define H0  0x67452301
#define H1  0xEFCDAB89
#define H2  0x98BADCFE
#define H3  0x10325476
#define H4  0xC3D2E1F0

std::vector<uint> words(const std::string& s) {
  std::vector<uint> words;
  words.reserve(16);

  for (int i = 0; i < (512 >> 2); i += 8) { // i=0..127
    std::string chunk = s.substr(i, 8);
    uint val = std::stoul(chunk, nullptr, 16);
    words.push_back(val);
  } 
  return words;
}


/**
* 80 word sequence
*/
uint W(const uint t, uint B, uint C, uint D) {
  uint TEMP = 0;
  if (t >= 0 && t <= 19) return (B & C) | ((~B) & D);
  else if (t >= 20 && t <= 39) return B ^ C ^ D;
  else if (t >= 40 && t <= 59) return (B & C) | (B & D) | (C & D);
  else return B ^ C ^ D;
}

std::string compress(const std::string& s) {
  uint A,B,C,D; // first 5-word buffer


  
  for (int t = 0; t < 80; t++) {

  }

}

std::string hex(const std::string& input) {
  std::ostringstream s;
  s << std::hex << std::setfill('0');

  for (unsigned char c : input) {
    s << std::setw(2) << static_cast<int>(c);
  }
  return s.str();
}

std::string sha1_decrypt(const std::string& m, const std::string& h) {
  std::string pwd_dir;
  pwd_dir.reserve(26 * 2 + 10);

  for (char c = '0'; c <= '9'; ++c) pwd_dir += c;
  for (char c = 'A'; c <= 'Z'; ++c) pwd_dir += c;
  for (char c = 'a'; c <= 'z'; ++c) pwd_dir += c;

  const unsigned int pwd_len = 6;
  const unsigned long long bit_len = (const unsigned long long)pwd_len * 8;
  std::string pwd_hashed, pwd;

  std::random_device rd;
  std::mt19937 gen(rd());
  std::uniform_int_distribution<> dist(0, pwd_dir.size() - 1);
  std::string pad;
  const unsigned int word_sz = 512;
  unsigned int pad_bits = 0;
  while ((bit_len + 64 + 1 + pad_bits) % 512 != 0) { // padding amount
    pad_bits++;

  }
  while (pad_bits % 4 > 0) pad_bits--; // if the padding isn't divisable by 8, decrement
  pad_bits /= 4; // padding 0's needed for a hexadecimal string

  //while (pwd_hashed != h) {
    pwd.reserve(pwd_len);

    for (int i = 0; i < pwd_len; i++) {
      pwd += pwd_dir[dist(gen)];
    }
    pwd_hashed = hex(pwd);
    pwd_hashed.append("8");

  for (int i = 0; i < pad_bits; i++) {
    pwd_hashed.append("0");
  }

  std::string out;
  out.reserve(pwd_hashed.size() + (pwd_hashed.size() / 8));
  
  std::stringstream s;
  s << std::hex << bit_len;
  std::string bit_len_str = s.str();
  while (bit_len_str.size() < 16) bit_len_str.insert(0, "0");
  pwd_hashed.append(bit_len_str);
 

  for (int i = 0; i < pwd_hashed.size(); i += 8) {
    out.append(pwd_hashed.substr(i, 8));
    if (i + 8 < pwd_hashed.size()) out.push_back(' ');
    if (i + 8 < pwd_hashed.size() && (i + 8) % 32 == 0) out.append("\n");
  }

  std::cout << "pwd hashed:\n" << out << "\nwith size: " << pwd_hashed.size() << std::endl;

  std::vector<uint> W = words(pwd_hashed);
  for(uint s : W) std::cout << "Word: " << s << std::endl;
  
  //}

  return pwd;

}

int main(void) {
  std::string hash = "dbc3337f151da4276572aaaa424cddb0d89a5422";
  
  std::string pwd, pwd_hash;

  sha1_decrypt(pwd, hash);
  //std::string pwd_decrypt = sha1_decrypt(pwd, hash);
  
  return 0;


}
