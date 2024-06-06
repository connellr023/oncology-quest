import { EntryStructure, UserTask } from "../models/tasks"
import jsPDF from "jspdf"

const useExportProgress = () => {
    const exportProgress = (name: string, tasks: Record<number, UserTask>, entries: EntryStructure) => {
        const offset = 15
        const indent = 15
        const multiplier = 10
        const spacing = 7

        const pdf = new jsPDF({
            format: "a4",
            orientation: "p"
        })

        pdf.setFontSize(10)
        pdf.text(`"${name}" Export`, 15, 10)

        let currentHeight = 0
        let lineCount = 0

        const checkPageBreak = () => {
            if (currentHeight > 3000) {
                pdf.addPage()

                lineCount = 0
                currentHeight = 0
            }
        }

        entries.forEach((supertask) => {
            checkPageBreak()

            const y = offset + spacing + (lineCount * multiplier)
            pdf.text(supertask.entry.title, indent, y)

            supertask.children.forEach((task) => {
                checkPageBreak()

                const y = offset + (spacing * 2) + (lineCount * multiplier)
                pdf.text(`- ${task.entry.title}`, indent * 2, y)

                task.children.forEach((subtask) => {
                    checkPageBreak()

                    const y = offset + (spacing * 3) + (lineCount * multiplier)
                    pdf.text(`- ${subtask.title}${tasks[subtask.id]?.isCompleted ? " - Complete" : ""}`, indent * 3, y)

                    const comment = tasks[subtask.id]?.comment
                    if (comment) {
                        checkPageBreak()
                    
                        const maxWidth = pdf.internal.pageSize.width - (indent * 5);
                        const splitComment = pdf.splitTextToSize(`- ${comment}`, maxWidth);
                    
                        splitComment.forEach((line: string, index: number) => {
                            const y = offset + (spacing * 4) + ((lineCount + index) * multiplier);
                            pdf.text(line, indent * 4, y);
                        });
                    
                        currentHeight += offset + (spacing * 4) + ((lineCount + splitComment.length - 1) * multiplier);
                        lineCount += splitComment.length;
                    }

                    currentHeight += y
                    lineCount += 1
                })

                currentHeight += y
                lineCount += 1
            })

            currentHeight += y
            lineCount += 1
        })

        pdf.save(`${name.toLowerCase().replace(" ", "-")}-export.pdf`)
    }
    
    return {
        exportProgress
    }
}

export default useExportProgress