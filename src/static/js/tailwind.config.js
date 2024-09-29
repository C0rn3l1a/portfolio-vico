/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./src/templates/**/*.{html,js}"],
    theme: {
        fontSize: {
            'sm': ['12px', {
                lineHeight: '20px',
                fontWeight: '400',
            }],
            'md': ['14px', {
                lineHeight: '22px',
                fontWeight: '400',
            }],
            'lg': ['16px', {
                lineHeight: '24px',
                fontWeight: '400',
            }],
            'xl': ['16px', {
                lineHeight: '50px',
                fontWeight: '600',
            }],
            '2xl': ['20px', {
                lineHeight: '50px',
                fontWeight: '600',
            }],
            '3xl': ['30px', {
                lineHeight: '50px',
                fontWeight: '600',
            }],
            '4xl': ['36px', {
                lineHeight: '50px',
                fontWeight: '600',
            }],
            '5xl': ['59px', {
                lineHeight: '50px',
                fontWeight: '600',
            }],
        },
        extend: {
            colors: {
                primary: {
                    50: '#E7EAF4',
                    100: '#CFD6EA',
                    200: '#A0AED4',
                    300: '#7487BD',
                    400: '#4B60A5',
                    500: '#26398D',
                    600: '#192867',
                    700: '#0D1744',
                    800: '#040923',
                },
                secondary: {
                    50: '#FFF6EC',
                    100: '#FFEDD9',
                    200: '#FFDBB2',
                    300: '#FFC889',
                    400: '#FFB45B',
                    500: '#FFA000',
                    600: '#BE7600',
                    700: '#804E00',
                    800: '#482900',
                }
            },
        },
    },
    plugins: [],
}
