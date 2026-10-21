export const formatIdDate = (dateInput: any): string => {
	if (!dateInput) return "-";

	try {
		const date = new Date(dateInput);

		// Check if the date is invalid
		if (isNaN(date.getTime())) return "-";

		return new Intl.DateTimeFormat("id-ID", {
			weekday: "long",
			day: "2-digit",
			month: "short",
			year: "numeric",
			hour: "2-digit",
			minute: "2-digit",
			hour12: false,
		}).format(date);
	} catch (error) {
		console.error("Error formatting date:", error);
		return "-";
	}
};
