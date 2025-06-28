#ifndef ROUGHGUARDWINDOW_H
#define ROUGHGUARDWINDOW_H

#include <QWidget>

class QPushButton;
class QLineEdit;
class QLabel;
class QVBoxLayout;
class QHBoxLayout;

class RoughGuardWindow : public QWidget
{
    Q_OBJECT

public:
    explicit RoughGuardWindow(QWidget *parent = nullptr);

private slots:
    void onAddFileClicked();
    void onSendClicked();

private:
    QPushButton* m_addPGNButton;
    QLineEdit* m_pathLineEdit;
    QLineEdit* m_resultLineEdit;
    QPushButton* m_sendButton;
    QVBoxLayout* m_mainLayout;
};

#endif // HOOKLENSWINDOW_H
