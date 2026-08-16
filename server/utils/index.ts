export const toMoney = (val: number) => {
	if (val) {
		const x = val;
		return x.toLocaleString("id-ID");
	} else {
		return "-";
	}
};
