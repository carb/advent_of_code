package main

import (
	"fmt"
	"strconv"
)

func mustAtoi(s string) int {
	i, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}
	return i
}

var uniqCharCount = 14

func process_input(input string) {
	for i := range input {
		a := map[byte]struct{}{}
		for j := 0; j < uniqCharCount; j++ {
			a[input[i+j]] = struct{}{}
		}
		if len(a) == uniqCharCount {
			fmt.Println(i + uniqCharCount)
			break
		}
	}

	return
}

func main() {
	// part 1
	process_input(testSignal)
	process_input(signal)
	// part 2
}

var testSignal = `bvwbjplbgvbhsrlpgdmjqwftvncz`

var signal = `dcbcsbblhhgdgssmcmqccdwdvwdvvcfvcfvvpvmmwccdqddshhdppcfpfbbfggfjfvvhzhmmsqswsttcgtctggjllhnnqbnnnldlglplvlbvbzztpzzvpphshwwfhhgssvgsvvpfvfzznbznndqqtsqttnvtvqqqrgqrrgzzlqzzbvzbzmzffnbfnbffcggdcggqhqpqzpqzqbbqvbqbhhtzhhrrrprlrclrcrssqspqpbblggfvvbpvbbssvttsztzpzjjwffnpnfnrrqhqgqrrzqzbzzdpdjpplpclpptltztjtztqthtmmmpsmmpjpqpvvshhsfshhrvrdvdggwssdbbzbttlmlmpprmrmlrrmqrqsshjhjljzjnjllbbvjvnnppqbppqqfffrbfbdfbbqppmtppfttqddtzzwbblqqbddgwgqwqzwwpzwzjjbccdjdwwrggrmrtmrtmrrvpptwwcdcwwsdshhjjhwjwqjwwqsshrrbsblssphspsjpssnsmsqswqwfwjwccdvccrqqnsqnsnwsnsmnmccvcqqfjjfljfljfjgjffjvffvjvhjhfhrfrlrrtpphbhwbbvmmnpndnlljdjfjsffvccnhchwhddrccljllmnnhmnmpnnhtnthnhvnhnwnqqjmmgzzvssphhjljbjwjbwwdnwwdvvzpvvhccvjjlclrrppgjgsshghvvzjvvpvvbjjqhjqqfcqffchcmmvdvtddftdtppfnppcnpnznwwpdpjpnpttlbbmmqfqmffqnfnbffbrbgbwggsmsbbfgbbfbzbtzbbvmmdlmlrlnlwwbswsjjpjhhwqwzqznqzqvvwccwvwwcbbpvvwcvcvzzhzchzccbnccjlljsjbjdbbvqbvvfdfgdfggzfzlzmzrznnqsnncmnnljjtcjcfftrthtrhrgrvvjggzpggrcrpcppggdfggpmmwnmnlmlglvvmjvvddsmsjsnjssqffffctczcjzjjlbbhpbbfdbdzbbsjbjnjbbqqjbqjbjjhwhssmvsschhhwppbjppllprpjrppdpfpgpjptjtptplpnptnptpjjmcmnnnhnjhjlhjlhjlhlwwhwmwjjbhhlfffrbbgvvqwvwswtstmmmfpmfpffmcfcrrqsqzqjjjnpnnztnznddlwdldsdbssstzstswwjtttmmwmsmwssvswszsjzsjjsffmccmfccqzzvpmbbbsqffgzdqbjtzhlqdzhlpwghlstcrcrffrnbwjnqgmbmpgttfmsswdqctlrpdnlsgnlldvbfpwtcptvbwftzcnbbscrrcpnwtmllcvsrmwzzlsdmfctdcwsqdlsnzgfpmzrnswhbqjhstztzzmzpcttsgsggnlhvjmcbbrhgqhsfmglpcbdvmmmnfbtbfrqbpcmttjtnwvznbshwrmznnsvpjqjntlzspljnbwtjcqztsfcqlrggrpzjgjsvqqcrmrjmzwdsshqfhbtfmlwmfvtbcgdmjgtcnphfmfmjlbjzrvjslccftnwcchgdwjnlthlwgldjwqwgdptdjdmzdrrzcdpbfrtdgcspjtqdqvzswwdwrhggdrqjjgwwrbwhhlrpqmszvlvjfqptncjlscvzbgzgmsttlbhbfrctnsphjcrcwlhcgrcrsjbrjvptgfbjgjrvtzmnhpzcgptbmrgvstsltnctjphsjdwpdqblfswzfhgjbpfrptlmhfwcpdlzqccgtdvbzhwngrhlqftmlhjprscflgzpflvvpfsjmlnmbzsrlrshvnsqrhhlqdlzhrcbjjjfrbqcdspwsmltcrtlbdnbnvhbbwgqdcncsztbfwztzdbqgcrnvndmpstpncbwvtctzdpmcpvrgqvjjztfwpvjtdqlvcvdpfzgcghsmbcwtzztmqwdpsprgsmfhphqsqmflrjdqzjscgzlnvcwcrlmpdscnhqpqjfdbdftqgttwntdbpnshdwnmwsrslfgnzlnwwwqgdbfnthhqtvbzsqgzjhhghtmvvfhlmlpbghnsvlttzsjlgndhdqmqqfdlqnbfscsnnqzcdwzdlqcnstcbsffghftqvwrsshgwmlnprhdnnwslwfwtmtfzdjwpmlvvdhjvdwrhvdsmpgrdpnqsjpqmhttrmdwllrmnbznwjwvvpjnnbnfzbdnhjbqqrnzgdqbspbqtwdpgsbwpzfdbpvzfjpsgmztnzrpvvhwlfscfvfpfblvplgdbhvjjpdjtnwrmvpjsphvglpsntvwtqwqvprcgwjltddpjngvmzfzhmqnnwglbzsbrcztpplpsmgmcfgzgpbtgsjrfvdzzcsthznvdpbwvdlcgdhncsjdvpcbbmtrqczctjljdfghsrvrsfjglqqhjttfdhqdqwhzhqrggsjwlldrntwmmbftgqjhpvvctpbtgltnttlhdqbsbwcqmctwlsnhmhncpmnzsllmjhcgnlfgvpcqrwzvsstgwbvjtnrlbblclzdrcwddrwnptqzgwdwtgrfwffpzjmwbqfmmrcfzfzjbwsslbhggtwtcrlhffzgfgrtljgnznnlgzwfmtwwqzhlthvpfclrtmrqfzrggbctzfsmjhrtwzpfrnhwwprnbwmvwvmvnzqpggmzgslctqbqdtvhgzjwzsnblqzmcpnpwllzhvzzmfqzfjlrsrcnjzqdzbpjftljtzvvmrjszvqllnhhgnrnqttnwlvllphjtnmlwqhcvmbsvnwtcdmhsmhdcwqwtvggcqfpdsrsscmhcvfzvdnffrfhdfmgbsghdbpwdwrdmlvsnzwcfchcqvccszdqrbnfvrpbftcwnjmczwgqzmtlmrthlhtjpmchltcmcqwgfgshtvtpmcmpbmlnjmhnpdsljjmjzddnlgtnjqztzsqqlhtqcscjvjncjvfcvsgjhqgzrmtjjcgvvmwswffmlcvlhqhbvrldrvbrfmmqcnqmfzlsghvclrdbsvcbqspgbzmjnlrdhvncnbcfmdlqrssggsmlwwglcjjzmcdwcnrgvvmgjcfbzlncmgqtllgldbdztbtfcjczqtjjlnpqmrgtjsmrbscvqlgqghfbgwccgwrjrzrsbbrqnjcqhllsqmrjtzmjnndwwmdjfhspjpgdcjjpdvwrsgjcfnpllnnnccvnfvqbpddgvbgbsbczmbzrbclczljdmbhpgmhwlqvnjzjwzzfjmsmcchjrqrtmhwlgjsptctlbtdnrqgntcvngcrqdqptfmhbvlhrqchdtwdwbrbqwtswzcrrfndldwmjbczppzrnncvvqsmpvvqcnsvhprhlmrhnjbwdvrbbwwdtmzrqttschrztgjcshlhfbmhmwrrmwgmfshpdhjwgdmcfdvqrmmwgmzbrlgbfltvlzmvqbgvhppdzglqbdlrhjnntnfvtmzjqccmqdbpjqphfggmjqdrndhclcfvqsjrsbcgdhcrbjsdjmwrzvpfpcjwjfdltztfzgmlzqzmztpvbflppgnhdzssznngfjggczdtdmcczjzfsnflpwqrbggmqdbbprfptzcdqvhrvszzjqqjlrlrdpwcnzhvgfhpcdbbgsfmtnszwbhwcdwdghqqctwqlqrlqbdwfvjnhcpmzchqfrwzhzgslzhncmrlrpzcjczvzwcvwcldbmscfqnnqnwwvrpfvjswqmhgmhgnmfzpsjmdhbpvsftccttvdpcdzcnzswqmtrwbctpbgmzrvrrshjjgdqsqrwfpcmsbvqhccvjqpztlttwjjdtbmwslschpqjjllvjcjwtmrtvvwdzglstvtpndmmzcpgqsvqgfdtqdjdctsbsbmqzqhtczhgqgwbdlrhjrwcmtbzfndsbnpnhmsdhtghwwzvtdwtscdnwzmrjrsrjvvbvrpbszchwbltrjbcqmlhqnzcfbhjqnsjghlnlbsrgzrrzfvslwpbqmgfswhgjdsdfrzsdhvdqvqfbcbvmbfhjfpzwlbspcbnvrgpfnmjbwbsnpqpqhjpsnwrlcmfhpdmjbvpnctcfqgdmwzblsnr`
