use rswind_extractor::ecma::EcmaExtractor;

fn main() {
    let input = r"
        let colors = {
            indigo: [
                'bg-indigo-50 text-indigo-600 hover:bg-indigo-200 hover:text-indigo-700 focus:ring-indigo-500',
                'text-indigo-300 group-hover:text-indigo-400',
            ],
            pink: [
                'bg-pink-50 text-pink-600 hover:bg-pink-100 hover:text-pink-700 focus:ring-pink-600',
                'text-pink-300 group-hover:text-pink-400',
            ],
            sky: [
                'bg-sky-50 text-sky-600 hover:bg-sky-100 hover:text-sky-700 focus:ring-sky-600',
                'text-sky-300 group-hover:text-sky-400',
            ],
            blue: [
                'bg-blue-50 text-blue-600 hover:bg-blue-100 hover:text-blue-700 focus:ring-blue-600',
                'text-blue-300 group-hover:text-blue-400',
            ],
            gray: [
                'bg-slate-100 text-slate-700 hover:bg-slate-200 hover:text-slate-900 focus:ring-slate-500',
                'text-slate-300 group-hover:text-slate-400',
            ],
        }
        const component = <Link
            className={clsx(
            'group inline-flex items-center h-9 rounded-full text-sm font-semibold whitespace-nowrap px-3 focus:outline-none focus:ring-2'
            )}
            {...props}
        )}
      {...props}
    >
    ";
    println!("{:#?}", EcmaExtractor::new(input).collect::<Vec<_>>());
}
