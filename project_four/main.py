from enum import Enum
import os

class Classification(Enum):
	AboutCats = 1
	NotAboutCats = 2

class PercentClassification:
	string: str
	aboutcats: float
	notaboutcats: float
	final_classification: Classification

	def __init__(self, string, ac, nac, fc):
		self.string = string
		self.aboutcats = ac
		self.notaboutcats = nac
		self.final_classification = fc

	def __str__(self):
		return f"================\n{self.string}\n================\nAboutCats: {self.aboutcats} NotAboutCats: {self.notaboutcats} Classification: {self.final_classification}\n"

class NaiveBayesClassifier:
	knowledge_base: dict[str, Classification]

	def __init__(self, knowledge_base = None):
		self.knowledge_base = knowledge_base or dict()

	def train(self, string, classification):
		self.knowledge_base[string] = classification

	def classify(self, string) -> PercentClassification:
		p_aboutcats = self.probability_of_classification(Classification.AboutCats) \
			* self.probability_of_words(string.split(), Classification.AboutCats)
		p_notaboutcats = self.probability_of_classification(Classification.NotAboutCats) \
			* self.probability_of_words(string.split(), Classification.NotAboutCats)
		fc = Classification.AboutCats if p_aboutcats > p_notaboutcats else Classification.NotAboutCats
		out = PercentClassification(string, p_aboutcats, p_notaboutcats, fc)
		return out

	def probability_of_classification(self, c: Classification):
		return sum([1 for _, cl in self.knowledge_base.items() if cl == c])/len(self.knowledge_base)

	def probability_of_words(self, words: list[str], c: Classification):
		summ = 1

		for word in words:
			a = sum([1 for key, cl in self.knowledge_base.items() if cl == c and word in key])
			# print(f"number {c} classifications: {sum([1 for key, cl in self.knowledge_base.items() if cl == c])}")
			prob = a / sum([1 for key, cl in self.knowledge_base.items() if cl == c])
			if prob == 0:
				prob = self.m_estimate(a, word, c)

			# print(f"Word: '{word}' has {prob*100}% chance that it is {c}")
			summ *= prob
			# print(summ)

		return summ

	def m_estimate(self, a, word, c):
		b = sum([1 for _, cl in self.knowledge_base.items() if cl == c])
		p = .5 # bc we only have two classifications
		m = len(self.knowledge_base)
		return (a + m*p)/(b+m)


def get_training_data(nac, nnac):
	out = []
	for i in range(nac):
		with open(f"training_data\\aboutcats\\file{i}.txt", "r") as f:
			# print(i)
			out.append((str(f.read()), Classification.AboutCats))
	for i in range(nnac):
		# print(i)
		with open(f"training_data\\notaboutcats\\file{i}.txt", "r") as f:
			out.append((str(f.read()), Classification.NotAboutCats))
	return out

def main():
	nbc = NaiveBayesClassifier()
	aboutcat_files = len(os.listdir("training_data\\aboutcats\\"))
	notaboutcat_files = len(os.listdir("training_data\\notaboutcats\\"))

	training_data = get_training_data(aboutcat_files, notaboutcat_files)

	for string, cl in training_data:
		nbc.train(string, cl)

	print(nbc.classify("my cat tom slays"))
	print(nbc.classify("my dog is a dog"))
	print(nbc.classify("A cat’s nutritional requirements change through different stages of life. These stages include kittenhood, adulthood, pregnancy, and lactation. The nutritional claim on the cat food label should state the stage of a cat’s life cycle"))
	print(nbc.classify("Our dogs love meats and fats, but overly rich foods don’t always love them back. Overindulging can irritate your dog’s pancreas and cause pancreatitis."))
	print(nbc.classify("Here is some text comepletely unrelated lets see what the nbc says"))

if __name__ == "__main__":
	main()