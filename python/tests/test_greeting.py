import unittest

from greeting import greeting


class GreetingTest(unittest.TestCase):
    def test_greets_someone_by_name(self):
        self.assertEqual(greeting("Ada"), "Hello, Ada!")

    def test_greets_a_stranger_without_a_name(self):
        self.assertEqual(greeting("  "), "Hello, stranger!")


if __name__ == "__main__":
    unittest.main()
