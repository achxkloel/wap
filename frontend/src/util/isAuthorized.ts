export const isAuthorized = (): boolean => {
    const token = localStorage.getItem('auth_token');
    return !!token;
};
