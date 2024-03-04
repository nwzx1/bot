#include <iostream>
using namespace std;

class Calc {

protected:
  auto add(int num1, int num2) -> int { return num1 + num2; };

  auto sub(int num1, int num2) -> int { return num1 - num2; };

  auto div(int num1, int num2) -> int { return num1 / num2; };

  auto mul(int num1, int num2) -> int { return num1 * num2; };
};

class calc : Calc {
public:
  void run() {

    int mode;
    int num1 = 0, num2 = 0;
    do {

      cout << "------------------------------------------------" << endl;
      cout << "------------------ Calculator ------------------" << endl;
      cout << "------------------------------------------------" << endl;
      cout << "1 : add \n2 : sub\n3 : div\n4 : mul\n0 : end" << endl;

      cout << "Enter the mode of calculation:> ";
      cin >> mode;

      switch (mode) {

      case 1:
        cout << "Enter num1 and num2 :> ";
        cin >> num1 >> num2;
        cout << "ans is: " << this->add(num1, num2) << endl;
        break;

      case 2:
        cout << "Enter num1 and num2 :> ";
        cin >> num1 >> num2;
        cout << "ans is: " << this->sub(num1, num2) << endl;
        break;

      case 3:
        cout << "Enter num1 and num2 :> ";
        cin >> num1 >> num2;
        cout << "ans is: " << this->div(num1, num2) << endl;
        break;

      case 4:
        cout << "Enter num1 and num2 :> ";
        cin >> num1 >> num2;
        cout << "ans is: " << this->mul(num1, num2) << endl;
        break;

      case 0:
        cout << "----------- good ----- THE - END ----- bye ------------"
             << endl;
        break;

      default:
        cout << "this number is not valid try again !!! " << endl;
      }

    } while (mode != 0);
  }
};

int main() {

  calc cal;
  cal.run();

  return 0;
}
